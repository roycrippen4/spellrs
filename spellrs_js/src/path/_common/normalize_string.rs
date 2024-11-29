use super::constants::{CHAR_DOT, CHAR_FORWARD_SLASH};
use crate::JS;

pub fn normalize_string(
    path: &str,
    allow_above_root: bool,
    separator: char,
    is_path_separator: impl Fn(u32) -> bool,
) -> String {
    let mut res = "".to_string();
    let mut last_segment_length = 0;
    let mut last_slash = -1;
    let mut dots = 0;
    let mut code: Option<i32> = None;

    for i in 0..=path.len() {
        if i < path.len() {
            code = Some(path.char_code_at(i))
        } else if code.is_some_and(|c| is_path_separator(c as u32)) {
            break;
        } else {
            code = Some(CHAR_FORWARD_SLASH as i32);
        }

        if code.is_some_and(|c| is_path_separator(c as u32)) {
            if last_slash == i as isize - 1 || dots == 1 {
                // noop
            } else if last_slash != i as isize - 1 && dots == 2 {
                if res.len() < 2
                    || last_segment_length != 2
                    || res.char_code_at(res.len() - 1) != CHAR_DOT as i32
                    || res.char_code_at(res.len() - 2) != CHAR_DOT as i32
                {
                    if res.len() > 2 {
                        let last_slash_idx = res.rfind(separator);
                        if let Some(last_slash_idx) = last_slash_idx {
                            res = res.slice(0, last_slash_idx as isize);
                            last_segment_length = res.len() as isize
                                - 1
                                - res.rfind(separator).map(|i| i as isize).unwrap_or(-1)
                        } else {
                            res = "".to_string();
                            last_segment_length = 0;
                        }
                        last_slash = i as isize;
                        dots = 0;
                        continue;
                    } else if res.len() == 2 || res.len() == 1 {
                        res = "".to_string();
                        last_segment_length = 0;
                        last_slash = i as isize;
                        dots = 0;
                        continue;
                    }
                }
                if allow_above_root {
                    if !res.is_empty() {
                        let part = format!("{separator}..");
                        res.push_str(&part);
                    } else {
                        res = "..".to_string();
                    }

                    last_segment_length = 2;
                }
            } else {
                if !res.is_empty() {
                    let slice = path.slice(last_slash + 1, i as isize);
                    let new_res = format!("{res}{separator}{slice}");
                    res = new_res;
                } else {
                    res = path.slice(last_slash + 1, i as isize).to_string();
                }
                last_segment_length = i as isize - last_slash - 1;
            }
            last_slash = i as isize;
            dots = 0;
        } else if code.is_some_and(|c| c == CHAR_DOT as i32) && dots != -1 {
            dots += 1;
        } else {
            dots = -1;
        }
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    fn is_separator(code: u32) -> bool {
        code == CHAR_FORWARD_SLASH
    }

    #[test]
    #[rustfmt::skip]
    fn test_normalize_string() {
        assert_eq!(normalize_string("", true, '/', is_separator), "");
        assert_eq!(normalize_string("", false, '/', is_separator), "");
        assert_eq!(normalize_string("a/../b", true, '/', is_separator), "b");
        assert_eq!(normalize_string("foo/bar/", true, '/', is_separator), "foo/bar");
        assert_eq!(normalize_string("/foo/bar", true, '/', is_separator), "foo/bar");
        assert_eq!(normalize_string("./foo/bar", true, '/', is_separator), "foo/bar");
        assert_eq!(normalize_string("../foo/bar/baz", true, '/', is_separator), "../foo/bar/baz");
        assert_eq!(normalize_string("/foo/../../bar", true, '/', is_separator), "../bar");
    }
}
