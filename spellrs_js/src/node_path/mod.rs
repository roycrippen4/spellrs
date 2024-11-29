mod _common;
mod posix;
mod windows;

use std::{env, fmt::Debug};

pub use posix::Posix;
pub use windows::Windows;

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

#[derive(Debug, PartialEq)]
pub struct ParsedPath {
    pub dir: String,
    pub root: String,
    pub base: String,
    pub name: String,
    pub ext: String,
}

pub trait PathInterface: Debug + Send + Sync {
    fn sep(&self) -> &'static str;
    fn resolve(&self, paths: &[&str]) -> String;
    fn parse(&self, path: &str) -> ParsedPath;
    fn normalize(&self, path: &str) -> String;
    fn relative(&self, from: &str, to: &str) -> String;
    fn is_absolute(&self, path: &str) -> bool;
}
