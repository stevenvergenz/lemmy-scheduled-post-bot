/*
Lemmy Scheduled Post Bot - makes Lemmy posts on a schedule
Copyright (C) 2024 Steven Vergenz
Licensed under the terms of the General Public License 3.0 or later
*/

use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Post {
    pub title: String,
    pub link: Option<String>,
    pub body: Option<String>,
    pub post_at: DateTime<Utc>,
}
