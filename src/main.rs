/*
Lemmy Scheduled Post Bot - makes Lemmy posts on a schedule
Copyright (C) 2024 Steven Vergenz
Licensed under the terms of the General Public License 3.0 or later
*/

use std::{env, process::exit};
use lemmy_scheduled_post_bot::process_posts_from_file;

const HELP: &str = "
lemmy-scheduled-post-bot <posts_file>

Arguments:
   posts_file - A toml file containing server information and one or more posts
";

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        print!("{HELP}");
        exit(1);
    }

    match process_posts_from_file(args.last().unwrap()).await {
        Err(err) => {
            eprintln!("Error: {err}");
            exit(1);
        },
        Ok(_) => { },
    }
}
