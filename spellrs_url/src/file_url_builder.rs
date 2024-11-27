#![allow(unused)]
use std::{env::consts, error::Error, slice::Windows};

use crate::{
    add_trailing_slash, file_url_to_path, is_url_like, normalize_windows_url,
    path_windows_drive_letter_to_upper, to_file_path_or_href, url_parent, url_to_url_relative,
    IsUrlLike, RE_WINDOWS_PATH, RE_WINDOWS_PATH_DRIVE_LETTER,
};

use once_cell::sync::Lazy;
use regex::Regex;
use url::Url;

pub static RE_PERCENT: Lazy<Regex> = Lazy::new(|| Regex::new(r"%").unwrap());
pub static RE_BACKSLASH: Lazy<Regex> = Lazy::new(|| Regex::new(r"\\").unwrap());
pub static RE_NEWLINE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\n").unwrap());
pub static RE_CARRIAGE_RETURN: Lazy<Regex> = Lazy::new(|| Regex::new(r"\r").unwrap());
pub static RE_TAB: Lazy<Regex> = Lazy::new(|| Regex::new(r"\t").unwrap());
pub static RE_QUESTION: Lazy<Regex> = Lazy::new(|| Regex::new(r"\?").unwrap());
pub static RE_HASH: Lazy<Regex> = Lazy::new(|| Regex::new(r"#").unwrap());

pub struct ParsedPath {
    pub root: String,
    pub dir: String,
    pub base: String,
    pub ext: String,
    pub name: String,
}

pub struct FileUrlBuilder {
    windows: bool,
    cwd: Url,
}

impl FileUrlBuilder {
    pub fn new(windows: Option<bool>, cwd: Option<Url>) -> Self {
        let is_windows = windows.unwrap_or(cfg!(windows));
        let cwd = cwd.unwrap_or_else(|| {
            let default_cwd = std::env::current_dir().expect("Failed to get cwd");
            Url::from_directory_path(default_cwd).expect("Failed to create Url from cwd")
        });

        Self {
            windows: is_windows,
            cwd,
        }
    }

    pub fn encode_path_chars(&self, filepath: &str) -> String {
        filepath
            .replace('%', "%25")
            .replace('\\', if self.windows { "%5C" } else { "\\" })
            .replace('\n', "%0A")
            .replace('\r', "%0D")
            .replace('\t', "%09")
    }

    pub fn normalize_filepath_for_url(&self, filepath: &str) -> String {
        let encoded = self.encode_path_chars(filepath);
        if self.windows {
            encoded.replace('\\', "/")
        } else {
            encoded.to_string()
        }
    }

    fn path_to_file_url(&self, pathname: &str) -> Url {
        let normalized_path = self.normalize_filepath_for_url(pathname);
        self.cwd.join(&normalized_path).expect("Invalid URL")
    }

    pub fn to_file_url() -> Url {
        todo!()
    }

    // fn _to_file_url(filename_or_url: impl Into<String>) -> Url {}

    fn get_fs_root_url(&self) {
        todo!()
    }

    // pub fn is_absolute(&self, filepath: &str) -> bool {
    //     is_url_like(filepath) || self.path.is_absolute(filepath)
    // }

    pub fn is_url_like(url: impl IsUrlLike) -> bool {
        is_url_like(url)
    }
}
