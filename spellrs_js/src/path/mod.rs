mod _common;
mod posix;
mod windows;

use std::{env, fmt::Debug};

pub use posix::Posix;
pub use windows::Windows;

/// If `std::env::consts::OS` resolves to "windows", then the windows implementation is used.
/// Otherwise, the posix implementation is used.
#[derive(Debug)]
pub struct DefaultPath;

impl PathInterface for DefaultPath {
    fn resolve(paths: &[&str]) -> String {
        match env::consts::OS {
            "windows" => Windows::resolve(paths),
            _ => Posix::resolve(paths),
        }
    }

    fn parse(path: &str) -> ParsedPath {
        match env::consts::OS {
            "windows" => Windows::parse(path),
            _ => Posix::parse(path),
        }
    }

    fn normalize(path: &str) -> String {
        match env::consts::OS {
            "windows" => Windows::normalize(path),
            _ => Posix::normalize(path),
        }
    }

    fn relative(from: &str, to: &str) -> String {
        match env::consts::OS {
            "windows" => Windows::relative(from, to),
            _ => Posix::relative(from, to),
        }
    }

    fn is_absolute(path: &str) -> bool {
        match env::consts::OS {
            "windows" => Windows::is_absolute(path),
            _ => Posix::is_absolute(path),
        }
    }

    fn sep() -> &'static str {
        match env::consts::OS {
            "windows" => Windows::sep(),
            _ => Posix::sep(),
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

pub trait PathInterface: Debug {
    fn sep() -> &'static str;
    fn resolve(paths: &[&str]) -> String;
    fn parse(path: &str) -> ParsedPath;
    fn normalize(path: &str) -> String;
    fn relative(from: &str, to: &str) -> String;
    fn is_absolute(path: &str) -> bool;
}
