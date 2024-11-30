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

use super::{ParsedPath, PathInterface, Res};

/// Posix implementation of the NodeJS path module
#[derive(Debug)]
pub struct Posix;

impl Posix {
    pub fn to_file_url(path: &str) -> Res<Url> {
        to_file_url(path)
    }
}

impl PathInterface for Posix {
    fn sep(&self) -> &'static str {
        "/"
    }

    fn parse(&self, path: &str) -> ParsedPath {
        parse(path)
    }

    fn resolve(&self, paths: &[&str]) -> String {
        resolve(paths)
    }

    fn normalize(&self, path: &str) -> String {
        normalize(path)
    }

    fn relative(&self, from: &str, to: &str) -> String {
        relative(from, to)
    }

    fn is_absolute(&self, path: &str) -> bool {
        is_absolute(path)
    }
}
