use crate::node_path::_common::constants::{
    CHAR_BACKWARD_SLASH, CHAR_FORWARD_SLASH, CHAR_LOWERCASE_A, CHAR_LOWERCASE_Z, CHAR_UPPERCASE_A,
    CHAR_UPPERCASE_Z,
};

pub(crate) fn is_path_separator(code: u32) -> bool {
    code == CHAR_FORWARD_SLASH || code == CHAR_BACKWARD_SLASH
}

pub(crate) fn is_windows_device_root(code: u32) -> bool {
    (CHAR_LOWERCASE_A..=CHAR_LOWERCASE_Z).contains(&code)
        || (CHAR_UPPERCASE_A..=CHAR_UPPERCASE_Z).contains(&code)
}
