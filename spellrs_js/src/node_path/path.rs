use std::env;

use url::Url;

use super::{ParsedPath, PathInterface, Posix, Res, Windows};

#[derive(Debug)]
pub struct NodePath {
    pub win32: Windows,
    pub posix: Posix,
}

impl NodePath {
    pub fn new() -> Self {
        Self {
            win32: Windows,
            posix: Posix,
        }
    }

    pub fn to_file_url(path: &str) -> Res<Url> {
        match cfg!(windows) {
            true => Windows::to_file_url(path),
            false => Posix::to_file_url(path),
        }
    }
}

impl Default for NodePath {
    fn default() -> Self {
        Self::new()
    }
}

impl PathInterface for NodePath {
    fn resolve(&self, paths: &[&str]) -> String {
        match env::consts::OS {
            "windows" => Windows.resolve(paths),
            _ => Posix.resolve(paths),
        }
    }

    fn parse(&self, path: &str) -> ParsedPath {
        match env::consts::OS {
            "windows" => Windows.parse(path),
            _ => Posix.parse(path),
        }
    }

    fn normalize(&self, path: &str) -> String {
        match env::consts::OS {
            "windows" => Windows.normalize(path),
            _ => Posix.normalize(path),
        }
    }

    fn relative(&self, from: &str, to: &str) -> String {
        match env::consts::OS {
            "windows" => Windows.relative(from, to),
            _ => Posix.relative(from, to),
        }
    }

    fn is_absolute(&self, path: &str) -> bool {
        match env::consts::OS {
            "windows" => Windows.is_absolute(path),
            _ => Posix.is_absolute(path),
        }
    }

    fn sep(&self) -> &'static str {
        match env::consts::OS {
            "windows" => Windows.sep(),
            _ => Posix.sep(),
        }
    }
}
