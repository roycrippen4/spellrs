use std::env;

use url::Url;

use super::{ParsedPath, Posix, Res, Windows, _internal::path_to_file_url::path_to_file_url};

#[derive(Debug)]
pub struct NodePath {
    pub win32: Windows,
    pub posix: Posix,
}

impl Default for NodePath {
    fn default() -> Self {
        Self {
            win32: Windows,
            posix: Posix,
        }
    }
}

impl NodePath {
    pub fn path_to_file_url(&self, filepath: &str, windows: Option<bool>) -> Res<Url> {
        path_to_file_url(filepath, windows)
    }

    pub fn to_file_url(&self, path: &str) -> Res<Url> {
        match cfg!(windows) {
            true => Windows.to_file_url(path),
            false => Posix.to_file_url(path),
        }
    }

    pub fn resolve(&self, paths: &[&str]) -> String {
        match env::consts::OS {
            "windows" => Windows.resolve(paths),
            _ => Posix.resolve(paths),
        }
    }

    pub fn parse(&self, path: &str) -> ParsedPath {
        match env::consts::OS {
            "windows" => Windows.parse(path),
            _ => Posix.parse(path),
        }
    }

    pub fn normalize(&self, path: &str) -> String {
        match env::consts::OS {
            "windows" => Windows.normalize(path),
            _ => Posix.normalize(path),
        }
    }

    pub fn relative(&self, from: &str, to: &str) -> String {
        match env::consts::OS {
            "windows" => Windows.relative(from, to),
            _ => Posix.relative(from, to),
        }
    }

    pub fn is_absolute(&self, path: &str) -> bool {
        match env::consts::OS {
            "windows" => Windows.is_absolute(path),
            _ => Posix.is_absolute(path),
        }
    }

    pub fn sep(&self) -> &'static str {
        match env::consts::OS {
            "windows" => Windows.sep(),
            _ => Posix.sep(),
        }
    }
}
