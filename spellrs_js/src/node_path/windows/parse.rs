use crate::{
    node_path::_common::constants::{CHAR_COLON, CHAR_DOT},
    ParsedPath, JS,
};

use super::util::{is_path_separator, is_windows_device_root};

pub(crate) fn parse(path: &str) -> ParsedPath {
    let mut ret = ParsedPath {
        dir: "".to_string(),
        root: "".to_string(),
        base: "".to_string(),
        name: "".to_string(),
        ext: "".to_string(),
    };

    let len = path.len() as isize;
    if path.is_empty() {
        return ret;
    }

    let mut root_end: isize = 0;
    let mut code = path.char_code_at(0);

    if len > 1 {
        if is_path_separator(code as u32) {
            root_end = 1;
            if is_path_separator(path.char_code_at(1) as u32) {
                let mut j: isize = 2;
                let mut last = j;

                while j < len {
                    if is_path_separator(path.char_code_at(j as usize) as u32) {
                        break;
                    }
                    j += 1;
                }

                if j < len && j != last {
                    last = j;

                    while j < len {
                        if !is_path_separator(path.char_code_at(j as usize) as u32) {
                            break;
                        }
                        j += 1;
                    }

                    if j < len && j != last {
                        last = j;

                        while j < len {
                            if is_path_separator(path.char_code_at(j as usize) as u32) {
                                break;
                            }
                            j += 1;
                        }
                        if j == len {
                            root_end = j;
                        } else if j != last {
                            root_end = j + 1;
                        }
                    }
                }
            }
        } else if is_windows_device_root(code as u32) && path.char_code_at(1) as u32 == CHAR_COLON {
            root_end = 2;
            if len > 2 {
                if is_path_separator(path.char_code_at(2) as u32) {
                    if len == 3 {
                        ret.root = path.to_string();
                        ret.dir = path.to_string();
                        return ret;
                    }
                    root_end = 3;
                }
            } else {
                ret.root = path.to_string();
                ret.base = "\\".to_string();
                return ret;
            }
        }
    } else if is_path_separator(code as u32) {
        ret.root = path.to_string();
        ret.dir = path.to_string();
        ret.base = "\\".to_string();
        return ret;
    }

    if root_end > 0 {
        ret.root = path.slice(0, root_end).to_string();
    }

    let mut start_dot = -1;
    let mut start_part = root_end;
    let mut end = -1;
    let mut matched_slash = true;
    let mut i = (path.len() - 1) as isize;
    let mut pre_dot_state = 0;

    while i >= root_end {
        code = path.char_code_at(i as usize);
        if is_path_separator(code as u32) {
            if !matched_slash {
                start_part = i + 1;
                break;
            }
            continue;
        }
        if end == -1 {
            matched_slash = false;
            end = i + 1;
        }
        if code as u32 == CHAR_DOT {
            if start_dot == -1 {
                start_dot = i;
            }
        } else if pre_dot_state != -1 {
            pre_dot_state = 1;
        } else if start_dot != -1 {
            pre_dot_state = -1;
        }
        i -= 1;
    }

    if start_dot == -1
        || end == -1
        || pre_dot_state == 0
        || (pre_dot_state == 1 && start_dot == end - 1 && start_dot == start_part + 1)
    {
        if end != -1 {
            let slice = path.slice(start_part, end);
            ret.base = slice.to_string();
            ret.name = slice.to_string();
        }
    } else {
        ret.name = path.slice(start_part, start_dot).to_string();
        ret.base = path.slice(start_part, end).to_string();
        ret.ext = path.slice(start_dot, end).to_string();
    }

    ret.base = match !ret.base.is_empty() {
        true => ret.base,
        false => "\\".to_string(),
    };

    if start_part > 0 && start_part != root_end {
        ret.dir = path.slice(0, start_part - 1).to_string();
    } else {
        ret.dir = ret.root.clone();
    }

    ret
}

#[cfg(test)]
mod test {
    use crate::ParsedPath;

    use super::parse;

    #[test]
    fn test_windows_parse() {
        let parsed = parse("\\\\server\\share");
        let expected = ParsedPath {
            base: "\\".into(),
            dir: "\\\\server\\share".into(),
            ext: "".into(),
            name: "".into(),
            root: "\\\\server\\share".into(),
        };

        assert_eq!(parsed, expected);
    }
}
