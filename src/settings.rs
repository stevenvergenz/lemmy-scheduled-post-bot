/*
Lemmy Scheduled Post Bot - makes Lemmy posts on a schedule
Copyright (C) 2024 Steven Vergenz
Licensed under the terms of the General Public License 3.0 or later
*/

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub instance: String,
    pub community: String,
    pub username_or_email: String,
    pub password: String,
    pub enabled: Option<bool>,
}
