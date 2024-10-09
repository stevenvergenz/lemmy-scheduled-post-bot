/*
Lemmy Scheduled Post Bot - makes Lemmy posts on a schedule
Copyright (C) 2024 Steven Vergenz
Licensed under the terms of the General Public License 3.0 or later
*/

mod config;
mod error;
mod post;
mod resolve_default;
mod settings;
mod template;
mod util;

use std::fs;
use chrono::Utc;
use lemmy_client::{
    lemmy_api_common::{
        community::GetCommunity, lemmy_db_schema::{newtypes::CommunityId, SortType}, lemmy_db_views::structs::PostView, person::{Login, LoginResponse}, post::{CreatePost, GetPosts, GetPostsResponse}, LemmyErrorType
    }, ClientOptions, LemmyClient, LemmyRequest
};
use config::Config;
use post::Post;
use settings::Settings;

pub async fn process_posts_from_file(config_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string(config_file)?;
    let config: Config = toml::from_str(&config_str)?;

    match process_posts(config).await {
        Err(e) => Err(e),
        Ok(_) => Ok(()),
    }
}

pub async fn process_posts(Config { settings, defaults, post }: Config) -> Result<(), Box<error::Error>> {
    // identify the post definition in the config whose scheduled post time most recently passed
    let now = Utc::now();
    let post = post.into_iter()
        .filter(|p| p.post_at.cmp(&now).is_le())
        .min_by_key(|p| now - p.post_at);

    let post = match post {
        // if nothing to post, just exit
        None => {
            println!("Nothing to post");
            return Ok(());
        }
        Some(p) => p,
    };

    let post = post.evaluate(defaults.as_ref());

    if !settings.enabled.unwrap_or(false) {
        println!("Next post is {}", util::fos(&post.options.title));
        return Ok(());
    }

    let mut client = LemmyClient::new(ClientOptions {
        domain: settings.instance.clone(),
        secure: true,
    });

    login(&settings, &mut client).await?;

    // verify community exists
    let community_id = match get_community_id(&settings.community, &client).await {
        Err(err) => return error::from_lemmy_error(err),
        Ok(c) => c,
    };

    // verify it hasn't already been posted
    let check_res = is_already_posted(&post, &client, &settings.community).await;
    if let Ok(true) = check_res {
        println!("Post with title {} has already been posted", util::fos(&post.options.title));
        return Ok(());
    }
    else if let Err(err) = check_res {
        return error::from_lemmy_error(err);
    }

    println!("Making post with title {} to {}/c/{}",
        util::fos(&post.options.title),
        settings.instance,
        settings.community);

    let res = client.create_post(LemmyRequest {
        body: CreatePost {
            community_id,
            name: post.options.title.expect("Posts must have a title!"),
            url: post.options.link,
            body: post.options.body,
            custom_thumbnail: post.options.thumbnail,
            alt_text: post.options.alt_text,
            ..Default::default()
        },
        jwt: None,
    });
    match res.await {
        Err(err) => error::from_lemmy_error(err),
        Ok(_) => Ok(()),
    }
}

async fn login(settings: &Settings, client: &mut LemmyClient) -> Result<(), Box<error::Error>> {

    // request log in
    let res = client.login(LemmyRequest {
        body: Login {
            username_or_email: settings.username_or_email.clone().into(),
            password: settings.password.clone().into(),
            totp_2fa_token: None,
        },
        jwt: None,
    }).await;

    // handle login errors
    match res {
        Ok(LoginResponse { jwt: Some(jwt), .. }) => {
            LemmyClient::headers_mut(client).insert(
                String::from("Authorization"),
                format!("Bearer {}", jwt.to_string()),
            );
        },
        Ok(res) => {
            return error::from_str(&format!(
                "Login pending, reg = {}, email = {}",
                res.registration_created,
                res.verify_email_sent,
            ));
        },
        Err(err) => {
            return error::from_lemmy_error(err);
        },
    };

    Ok(())
}

async fn get_community_id(name: &str, client: &LemmyClient) -> Result<CommunityId, LemmyErrorType> {
    let community = client.get_community(LemmyRequest {
        body: GetCommunity {
            name: Some(String::from(name)),
            id: None,
        },
        jwt: None,
    })
    .await?;

    Ok(community.community_view.community.id)
}

async fn is_already_posted(
    post: &Post,
    client: &LemmyClient,
    community: &str,
) -> Result<bool, LemmyErrorType> {
    // get the bot user info
    let my_id = client.get_site(LemmyRequest::empty()).await?
        .my_user.expect("Logged in but no user?").local_user_view.person.id;

    let mut req = LemmyRequest {
        body: GetPosts {
            community_name: Some(String::from(community)),
            sort: Some(SortType::New),
            page_cursor: None,
            ..Default::default()
        },
        jwt: None,
    };

    // gather potential posts
    let mut posts: Vec<PostView> = vec![];
    loop {
        // request a page of recent posts
        let recent_posts = match client.list_posts(req.clone()).await {
            Ok(GetPostsResponse { posts, next_page }) => {
                req.body.page_cursor = next_page;
                posts
            },
            Err(err) => return Err(err),
        };

        // stop looking immediately if the newest post in the page is older than our expected post time
        if recent_posts.first().is_none() {
            break;
        }
        else if let Some(first) = recent_posts.first() {
            if first.post.published < post.post_at {
                break;
            }
        }

        // stop looking after appending if last post is the same
        let mut is_last_page = false;
        if let Some(last) = recent_posts.last() {
            is_last_page = last.post.published < post.post_at;
        }

        // update the list of posts that might be it
        for possible in recent_posts.into_iter().filter(
            |p| p.creator.id == my_id && p.post.published >= post.post_at,
        ) {
            posts.push(possible);
        }

        // follow through on earlier check
        if is_last_page {
            break;
        }
    }

    Ok(posts.iter()
        .find(|p| {
            Some(&p.post.name) == post.options.title.as_ref()
        })
        .is_some(),
    )
}
