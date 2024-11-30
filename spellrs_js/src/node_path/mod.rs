mod _common;
mod path;
mod posix;
mod windows;

use std::{error::Error, fmt::Debug};

pub use path::NodePath;
pub use posix::Posix;
pub use windows::Windows;

pub(crate) type Res<T> = Result<T, Box<dyn Error>>;

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
