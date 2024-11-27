#![allow(unused)]
use std::{env::consts, error::Error};

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

pub trait PathInterface {
    fn sep(&self) -> &str;
    fn resolve(&self, paths: &[&str]) -> String;
    fn parse(&self, path: &str) -> ParsedPath;
    fn normalize(&self, path: &str) -> String;
    fn relative(&self, from: &str, to: &str) -> String;
    fn is_absolute(&self, path: &str) -> bool;
}

pub struct BuilderOptions {
    windows: Option<bool>,
    path: Option<Box<dyn PathInterface>>,
    cwd: Option<Url>,
}

pub struct FileUrlBuilder {
    windows: bool,
    path: Box<dyn PathInterface>,
    cwd: Url,
}

impl FileUrlBuilder {
    pub fn new(options: BuilderOptions) -> Result<Self, Box<dyn Error>> {
        todo!()
    }

    pub fn encode_path_chars(&self, filepath: &str) -> String {
        let mut filepath = RE_PERCENT.replace_all(filepath, "%25").to_string();

        if !self.windows && consts::OS != "windows" && filepath.contains("\\") {
            filepath = RE_BACKSLASH.replace_all(&filepath, "%5C").to_string();
        }

        filepath = RE_NEWLINE.replace_all(&filepath, "%0A").to_string();
        filepath = RE_CARRIAGE_RETURN.replace_all(&filepath, "%0D").to_string();
        RE_TAB.replace_all(&filepath, "%09").to_string()
    }

    pub fn normalize_filepath_for_url(&self, filepath: &str) -> String {
        let mut filepath = self.encode_path_chars(filepath);
        filepath = RE_QUESTION.replace_all(&filepath, "%3F").to_string();
        filepath = RE_HASH.replace_all(&filepath, "%23").to_string();
        let pathname = filepath.replace("\\", "/");

        RE_WINDOWS_PATH
            .replace(&pathname, |caps: &regex::Captures| {
                format!("/{}", &caps[0].to_uppercase())
            })
            .to_string()
    }

    pub fn to_file_url() -> Url {
        todo!()
    }

    fn _to_file_url(filename_or_url: impl Into<String>) -> Url {
        todo!()
    }

    fn get_fs_root_url(&self) {
        todo!()
    }

    pub fn is_absolute(&self, filepath: &str) -> bool {
        is_url_like(filepath) || self.path.is_absolute(filepath)
    }

    pub fn is_url_like(url: impl IsUrlLike) -> bool {
        is_url_like(url)
    }
}
