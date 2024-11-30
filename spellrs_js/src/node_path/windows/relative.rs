use crate::{node_path::_common::constants::CHAR_BACKWARD_SLASH, JS};

use super::resolve::resolve;

pub(crate) fn relative(from: &str, to: &str) -> String {
    let from_orig = resolve(&[from]);
    let to_orig = resolve(&[to]);

    if from_orig == to_orig {
        return "".to_string();
    }

    let from = from_orig.to_lowercase();
    let to = to_orig.to_lowercase();

    if from == to {
        return "".to_string();
    }

    let mut from_start = 0;
    let mut from_end = from.len();

    while from_start < from_end {
        if from.char_code_at(from_start) as u32 != CHAR_BACKWARD_SLASH {
            break;
        }
        from_start += 1
    }

    while from_end - 1 > from_start {
        if from.char_code_at(from_end - 1) as u32 != CHAR_BACKWARD_SLASH {
            break;
        }
        from_end -= 1;
    }
    let from_len = from_end - from_start;
    let mut to_start = 0;
    let mut to_end = to.len();

    while to_start < to_end {
        if to.char_code_at(to_start) as u32 != CHAR_BACKWARD_SLASH {
            break;
        }
        to_start += 1;
    }

    while to_end - 1 > to_start {
        if to.char_code_at(to_end - 1) as u32 != CHAR_BACKWARD_SLASH {
            break;
        }
        to_end -= 1;
    }
    let to_len = to_end - to_start;
    let length = match from_len < to_len {
        true => from_len,
        false => to_len,
    };
    let mut last_common_sep = None;
    let mut i = 0;
    while i <= length {
        if i == length {
            if to_len > length {
                if to.char_code_at(to_start + i) as u32 == CHAR_BACKWARD_SLASH {
                    return to_orig.slice((to_start + i + 1) as isize, to_orig.len() as isize);
                } else if i == 2 {
                    return to_orig.slice((to_start + i) as isize, to_orig.len() as isize);
                }
            }
            if from_len > length {
                if from.char_code_at(from_start + i) as u32 == CHAR_BACKWARD_SLASH {
                    last_common_sep = Some(i);
                } else if i == 2 {
                    last_common_sep = Some(3);
                }
            }
            break;
        }
        let from_code = from.char_code_at(from_start + i);
        let to_code = to.char_code_at(to_start + i);
        if from_code != to_code {
            break;
        } else if from_code as u32 == CHAR_BACKWARD_SLASH {
            last_common_sep = Some(i);
        }
        i += 1;
    }

    if i != length && last_common_sep.is_none() {
        return to_orig;
    }

    let mut out = "".to_string();
    let last_common_sep = last_common_sep.unwrap_or(0);

    let mut i = from_start + last_common_sep + 1;
    while i <= from_end {
        if i == from_end || from.char_code_at(i) as u32 == CHAR_BACKWARD_SLASH {
            if out.is_empty() {
                out += "..";
            } else {
                out += "\\..";
            }
        }
        i += 1;
    }

    if !out.is_empty() {
        return out + &to_orig.slice((to_start + last_common_sep) as isize, to_end as isize);
    }

    to_start = last_common_sep;
    if to_orig.char_code_at(to_start) as u32 == CHAR_BACKWARD_SLASH {
        to_start += 1;
    }

    to_orig.slice(to_start as isize, to_end as isize)
}

#[cfg(test)]
mod test {
    use super::relative;

    #[test]
    fn test_windows_relative() {
        let cases: Vec<((&str, &str), String)> = vec![
            (("/", "/"), "".into()),
            (("/foo/bar", "/foo/bar"), "".into()),
            (("/foo", "/foo/bar"), "bar".into()),
            (("/foo/bar", "/foo"), "..".into()),
            (("/foo/bar", "/foo/baz"), "..\\baz".into()),
            (("/foo/bar/baz", "/foo/bar/qux"), r"..\qux".into()),
            (("/foo/bar/baz", "/foo/qux/quux"), r"..\..\qux\quux".into()),
            (("/foo/bar", "/baz/qux"), r"\baz\qux".into()),
            (("/", "/foo"), "foo".into()),
            (("/foo", "/"), "..".into()),
            (("/", "/foo/bar"), "foo\\bar".into()),
            (("/foo/bar", "/"), "..\\..".into()),
            (("/foo/bar/baz", "\\foo\\bar"), "..".into()),
            (("/foo/bar", "/foo/bar/baz/qux"), "baz\\qux".into()),
            (("/foo/bar/..", "/foo/baz"), "baz".into()),
            (("/foo/bar/.", "/foo/bar/baz"), "baz".into()),
            (("/foo/../bar", "/bar/baz"), "baz".into()),
            (("/foo/./bar", "/foo/bar"), "".into()),
            (("/foo/bar/", "/foo/bar/baz/"), "baz".into()),
            (("/foo/bar/", "/foo/"), "..".into()),
            (("/foo", "/foo/bar/."), "bar".into()),
            (("/foo/.", "/foo/bar"), "bar".into()),
            (("/foo/..", "/bar"), "bar".into()),
            (("/foo/..", "/foo"), "foo".into()),
            (("/foo/bar", "/foo/bar/.."), "..".into()),
        ];

        for (i, ((from, to), expected)) in cases.iter().enumerate() {
            let result = relative(from, to);
            assert_eq!(
                &result, expected,
                "\n\nCASE {i} FAILED\nFROM    : \"{from}\"\nTO      : \"{to}\"\nresult  : {:?}\nexpected: {:?}\n\n",
                result, expected
            );
        }
    }
}
