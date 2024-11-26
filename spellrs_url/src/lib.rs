mod data_url;
mod default_file_url_builder;
mod file_url;
mod file_url_builder;
mod url;

pub use data_url::{is_data_url, url_basename};
pub use default_file_url_builder::{
    encode_path_chars, normalize_file_path_for_url, to_file_dir_url, to_file_url,
};
pub use file_url::{is_file_url, to_file_path_or_href};
pub use file_url_builder::{BuilderOptions, FileUrlBuilder, PathInterface};
pub use url::{
    add_trailing_slash, basename_of_url_pathname, has_protocol, is_not_url_like, is_url,
    is_url_like, to_url, url_dirname, url_parent, url_relative,
};
