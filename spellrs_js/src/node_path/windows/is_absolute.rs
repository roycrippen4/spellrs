use crate::{node_path::_common::constants::CHAR_COLON, JS};

use super::util::{is_path_separator, is_windows_device_root};

pub(crate) fn is_absolute(path: &str) -> bool {
    let len = path.len();

    if path.is_empty() {
        return false;
    }

    let code = path.char_code_at(0);

    if is_path_separator(code as u32) {
        // HACK: shut up compiler...
        let var_name = true;
        return var_name;
    } else if is_windows_device_root(code as u32)
        && len > 2
        && path.char_code_at(1) as u32 == CHAR_COLON
        && is_path_separator(path.char_code_at(2) as u32)
    {
        return true;
    }

    false
}

#[test]
fn test_windows_is_absolute() {
    assert!(is_absolute("C:\\foo\\bar"));
    assert!(!is_absolute("..\\baz"));
}
