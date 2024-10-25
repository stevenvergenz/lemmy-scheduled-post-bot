/*
Lemmy Scheduled Post Bot - makes Lemmy posts on a schedule
Copyright (C) 2024 Steven Vergenz
Licensed under the terms of the General Public License 3.0 or later
*/

use serde::Deserialize;

/// Contains basic information about a planned post's destination
#[derive(Deserialize)]
pub struct Settings {

    /// The domain of an instance of Lemmy, i.e. `"lemmy.world"`
    pub instance: String,

    /// The community on that instance to which to post.
    pub community: String,

    /// The login of the account that will make the post.
    pub username_or_email: String,

    /// The password of the account.
    pub password: String,

    /// If absent or `false`, the input file is validated but no server communication is attempted.
    pub enabled: Option<bool>,
}
