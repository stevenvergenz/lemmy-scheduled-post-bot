/*
Lemmy Scheduled Post Bot - makes Lemmy posts on a schedule
Copyright (C) 2024 Steven Vergenz
Licensed under the terms of the General Public License 3.0 or later
*/

use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::resolve_default;
use crate::template::Template;

#[derive(Deserialize, Debug)]
pub struct Post {
    pub post_at: DateTime<Utc>,

    #[serde(flatten)]
    pub options: PostOptions,
}

#[derive(Deserialize, Default, Debug)]
pub struct PostOptions {
    pub title: Option<String>,
    pub link: Option<String>,
    pub body: Option<String>,
    pub thumbnail: Option<String>,
    pub alt_text: Option<String>,

    // other fields for templating
    #[serde(flatten)]
    pub template_fields: HashMap<String, toml::Value>,
}

impl Post {
    pub fn evaluate(self, defaults: Option<&PostOptions>) -> Post {
        let Post {
            post_at,
            options: PostOptions {
                title,
                link,
                thumbnail,
                alt_text,
                body,
                template_fields,
            },
        } = self;

        let mut fields = HashMap::new();

        // add standard fields to the lookup
        if let Some(title) = resolve_default!(title, defaults.title) {
            fields.insert("title", toml::Value::String(title.clone()));
        }
        if let Some(link) = resolve_default!(link, defaults.link) {
            fields.insert("link", toml::Value::String(link.clone()));
        }
        if let Some(thumbnail) = resolve_default!(thumbnail, defaults.thumbnail) {
            fields.insert("thumbnail", toml::Value::String(thumbnail.clone()));
        }
        if let Some(alt_text) = resolve_default!(alt_text, defaults.alt_text) {
            fields.insert("alt_text", toml::Value::String(alt_text.clone()));
        }
        if let Some(body) = resolve_default!(body, defaults.body) {
            fields.insert("body", toml::Value::String(body.clone()));
        }

        // add per-post custom fields to lookup
        for (k, v) in template_fields.iter() {
            if !fields.contains_key(k.as_str()) {
                fields.insert(k, v.clone());
            }
        }

        // add default custom fields to lookup
        if let Some(defaults) = defaults {
            for (k, v) in defaults.template_fields.iter() {
                if !fields.contains_key(k.as_str()) {
                    fields.insert(k, v.clone());
                }
            }
        }

        Post {
            post_at,
            options: PostOptions {
                // evaluate template fields in standard fields
                title: resolve_default!(title, defaults.title)
                    .map(|x| Template(x, &fields).into()),
                link: resolve_default!(link, defaults.link)
                    .map(|x| Template(x, &fields).into()),
                thumbnail: resolve_default!(thumbnail, defaults.thumbnail)
                    .map(|x| Template(x, &fields).into()),
                alt_text: resolve_default!(alt_text, defaults.alt_text)
                    .map(|x| Template(x, &fields).into()),
                body: resolve_default!(body, defaults.body)
                    .map(|x| Template(x, &fields).into()),
                template_fields: HashMap::new(),
            },
        }
    }
}
