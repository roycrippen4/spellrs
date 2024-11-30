mod _common;
mod _internal;
mod path;
pub(crate) mod posix;
pub(crate) mod windows;

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
