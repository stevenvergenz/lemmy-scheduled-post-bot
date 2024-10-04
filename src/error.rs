/*
Lemmy Scheduled Post Bot - makes Lemmy posts on a schedule
Copyright (C) 2024 Steven Vergenz
Licensed under the terms of the General Public License 3.0 or later
*/

use std::convert::From;
use std::error;
use std::fmt::{self, Display, Formatter};
use lemmy_client::lemmy_api_common::LemmyErrorType;

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self {
            message: String::from(value),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for Error {}

pub fn from_str<T>(message: &str) -> Result<T, Box<Error>> {
    Err(Box::new(Error::from(message)))
}

pub fn from_lemmy_error<T>(err: LemmyErrorType) -> Result<T, Box<Error>> {
    match err {
        err => from_str(&format!("{}", err)),
    }
}
