mod data_url;
mod default_file_url_builder;
mod file_url;
mod file_url_builder;
pub mod traits;
mod url;

use std::fmt::Display;

use ::url::{ParseError, Url};
pub use data_url::{is_data_url, url_basename};
pub use default_file_url_builder::{
    encode_path_chars, normalize_file_path_for_url, to_file_dir_url, to_file_url,
};
pub use file_url::{
    file_url_to_path, is_file_url, path_windows_drive_letter_to_upper, to_file_path_or_href,
    RE_WINDOWS_PATH_DRIVE_LETTER,
};
pub use file_url_builder::FileUrlBuilder;
pub use traits::*;
pub use url::{
    add_trailing_slash, basename_of_url_pathname, has_protocol, is_not_url_like, is_url,
    is_url_like, normalize_windows_url, to_url, url_dirname, url_parent, url_relative,
    url_to_url_relative, RE_WINDOWS_PATH,
};

/// Many functions from this port accept strings or Urls as parameters.
/// This is my best attempt at keeping the implementation similar to the original
pub enum StUrl<'a> {
    Str(&'a str),
    String(String),
    Url(Url),
}

impl<'a> From<&'a str> for StUrl<'a> {
    fn from(s: &'a str) -> Self {
        Self::Str(s)
    }
}

impl<'a> From<Url> for StUrl<'a> {
    fn from(url: Url) -> Self {
        Self::Url(url)
    }
}

impl<'a> From<String> for StUrl<'a> {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl<'a> StUrl<'a> {
    pub fn as_str(&self) -> &str {
        match self {
            StUrl::Str(s) => s,
            StUrl::String(s) => s.as_str(),
            StUrl::Url(url) => url.as_str(),
        }
    }

    pub fn as_url(&self) -> Result<Url, ParseError> {
        match self {
            StUrl::Str(s) => Url::parse(s),
            StUrl::String(s) => Url::parse(s),
            StUrl::Url(url) => Ok(url.clone()),
        }
    }
}

impl<'a> Clone for StUrl<'a> {
    fn clone(&self) -> Self {
        match self {
            Self::Str(s) => Self::Str(s),
            Self::String(s) => Self::String(s.clone()),
            Self::Url(url) => Self::Url(url.clone()),
        }
    }
}

impl<'a> Display for StUrl<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StUrl::Str(s) => write!(f, "{s}"),
            StUrl::String(s) => write!(f, "{s}"),
            StUrl::Url(url) => write!(f, "{}", url.as_str()),
        }
    }
}
