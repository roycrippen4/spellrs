use std::env;

use crate::node_path::_common::constants::CHAR_FORWARD_SLASH;

pub(crate) fn is_posix_path_separator(code: u32) -> bool {
    code == CHAR_FORWARD_SLASH
}

pub(crate) fn posix_cwd() -> String {
    if cfg!(windows) {
        let cwd = env::current_dir().expect("Failed to get current directory");
        let path = cwd.to_str().expect("Failed to convert cwd to string");
        let posix_path = path.replace('\\', "/");

        if let Some(index) = posix_path.find('/') {
            return posix_path[index..].to_string();
        }

        posix_path
    } else {
        let cwd = env::current_dir().expect("Failed to get current directory");
        cwd.to_str()
            .expect("Failed to convert path to string")
            .to_string()
    }
}

pub(crate) fn escape_special_chars(input: &str) -> String {
    // handle null terminated bytes
    if input.contains(r"\0") {
        return input.replace(r"\0", r"\u0000");
    }

    if input.contains("\\") {
        return input.replace("\\", "\\\\");
    }

    input.to_string()
}
