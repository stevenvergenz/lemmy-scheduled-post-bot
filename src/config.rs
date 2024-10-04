/*
Lemmy Scheduled Post Bot - makes Lemmy posts on a schedule
Copyright (C) 2024 Steven Vergenz
Licensed under the terms of the General Public License 3.0 or later
*/

use serde::Deserialize;

use super::{post::Post, settings::Settings};

#[derive(Deserialize)]
pub struct Config {
    pub settings: Settings,
    pub post: Vec<Post>,
}