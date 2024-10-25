/*
Lemmy Scheduled Post Bot - makes Lemmy posts on a schedule
Copyright (C) 2024 Steven Vergenz
Licensed under the terms of the General Public License 3.0 or later
*/

use serde::Deserialize;

use super::{post::{Post, PostOptions}, settings::Settings};

/// Contains the necessary information to make a post to a Lemmy server
#[derive(Deserialize)]
pub struct Config {

    /// Post destination information and general settings
    pub settings: Settings,

    /// Fields in this post will be used if they are missing from a post
    pub defaults: Option<PostOptions>,

    /// A list of scheduled posts
    pub post: Vec<Post>,
}
