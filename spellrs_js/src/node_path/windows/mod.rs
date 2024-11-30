mod is_absolute;
mod normalize;
mod parse;
mod relative;
mod resolve;
mod to_file_url;
mod util;

use is_absolute::is_absolute;
use normalize::normalize;
use parse::parse;
use relative::relative;
use resolve::resolve;
use to_file_url::to_file_url;

use url::Url;

use super::{ParsedPath, Res, _internal::path_to_file_url::path_to_file_url};

/// Windows implementation of the NodeJS path module
#[derive(Debug)]
pub struct Windows;

impl Windows {
    pub fn path_to_file_url(&self, filepath: &str, windows: Option<bool>) -> Res<Url> {
        path_to_file_url(filepath, windows)
    }

    pub fn to_file_url(&self, path: &str) -> Res<Url> {
        to_file_url(path)
    }

    pub fn sep(&self) -> &'static str {
        r"\"
    }

    pub fn parse(&self, path: &str) -> ParsedPath {
        parse(path)
    }

    pub fn resolve(&self, paths: &[&str]) -> String {
        resolve(paths)
    }

    pub fn normalize(&self, path: &str) -> String {
        normalize(path)
    }

    pub fn relative(&self, from: &str, to: &str) -> String {
        relative(from, to)
    }

    pub fn is_absolute(&self, path: &str) -> bool {
        is_absolute(path)
    }
}
