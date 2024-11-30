pub mod globals;
mod js;
mod node_path;

pub use js::JS;
pub use node_path::{NodePath, ParsedPath, Posix, Windows};
