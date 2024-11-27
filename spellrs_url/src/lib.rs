mod data_url;
mod default_file_url_builder;
mod file_url;
mod file_url_builder;
pub mod traits;
mod url;

pub use data_url::{is_data_url, url_basename};
pub use default_file_url_builder::{
    encode_path_chars, normalize_file_path_for_url, to_file_dir_url, to_file_url,
};
pub use file_url::{
    file_url_to_path, is_file_url, path_windows_drive_letter_to_upper, to_file_path_or_href,
    RE_WINDOWS_PATH_DRIVE_LETTER,
};
pub use file_url_builder::{BuilderOptions, FileUrlBuilder, PathInterface};
pub use traits::*;
pub use url::{
    add_trailing_slash, basename_of_url_pathname, decode_uri_component, has_protocol,
    is_not_url_like, is_url, is_url_like, normalize_windows_url, to_url, url_dirname, url_parent,
    url_relative, url_to_url_relative, RE_WINDOWS_PATH,
};
