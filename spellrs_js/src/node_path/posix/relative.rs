use crate::JS;

use super::{resolve, util::is_posix_path_separator};

pub(crate) fn relative(from: &str, to: &str) -> String {
    if from == to {
        return "".to_string();
    }

    let from = resolve(&[from]);
    let to = resolve(&[to]);

    if from == to {
        return "".to_string();
    }

    let mut from_start = 1;
    let from_end = from.len();

    while from_start < from_end {
        if !is_posix_path_separator(to.char_code_at(from_start) as u32) {
            break;
        }
        from_start += 1;
    }
    let from_len = from_end - from_start;

    // trim any leading backslashes
    let mut to_start = 1;
    let to_end = to.len();
    while to_start < to_end {
        if !is_posix_path_separator(to.char_code_at(to_start) as u32) {
            break;
        }
        to_start += 1;
    }
    let to_len = to_end - to_start;

    let length = if from_len < to_len { from_len } else { to_len };
    let mut last_common_sep = None;
    let mut i = 0;

    while i <= length {
        if i == length {
            if to_len > length {
                if is_posix_path_separator(to.char_code_at(to_start + i) as u32) {
                    // We get here if `from` is the exact base path for `to`.
                    // For example: from='/foo/bar'; to='/foo/bar/baz'
                    return to.slice((to_start + i) as isize + 1, to.len() as isize);
                } else if i == 0 {
                    // We get here if `from` is the root
                    // For example: from='/'; to='/foo'
                    return to.slice((to_start + i) as isize, to.len() as isize);
                }
            } else if from_len > length {
                if is_posix_path_separator(from.char_code_at(from_start + i) as u32) {
                    // We get here if `to` is the exact base path for `from`.
                    // For example: from='/foo/bar/baz'; to='/foo/bar'
                    last_common_sep = Some(i);
                } else if i == 0 {
                    // We get here if `to` is the root.
                    // For example: from='/foo'; to='/'
                    last_common_sep = Some(0);
                }
            }
            break;
        }
        let from_code = from.char_code_at(from_start + i);
        let to_code = to.char_code_at(to_start + i);

        if from_code != to_code {
            break;
        } else if is_posix_path_separator(from_code as u32) {
            last_common_sep = Some(i);
        }
        i += 1
    }

    let mut out = "".to_string();

    // Generate the relative path based on the path difference between `to`
    // and `from`
    let mut i = match last_common_sep {
        Some(last_common_sep) => from_start + last_common_sep + 1,
        None => from_start,
    };
    while i <= from_end {
        if i == from_end || is_posix_path_separator(from.char_code_at(i) as u32) {
            if out.is_empty() {
                out += "..";
            } else {
                out += "/.."
            }
        }
        i += 1;
    }

    if !out.is_empty() {
        let start_idx = match last_common_sep {
            Some(last_common_sep) => to_start + last_common_sep,
            None => to_start - 1,
        };
        let slice = to.slice(start_idx as isize, to.len() as isize);
        return out + &slice;
    }

    to_start = match last_common_sep {
        Some(last_common_sep) => to_start + last_common_sep,
        None => to_start - 1,
    };

    if is_posix_path_separator(to.char_code_at(to_start) as u32) {
        to_start += 1;
    }

    to.slice(to_start as isize, to.len() as isize)
}
#[cfg(test)]
mod test {
    use super::relative;

    #[test]
    fn test_posix_relative() {
        let cases: Vec<((&str, &str), String)> = vec![
            (("/", "/"), "".into()),
            (("/foo/bar", "/foo/bar"), "".into()),
            (("/foo", "/foo/bar"), "bar".into()),
            (("/foo/bar", "/foo"), "..".into()),
            (("/foo/bar", "/foo/baz"), "../baz".into()),
            (("/foo/bar/baz", "/foo/bar/qux"), "../qux".into()),
            (("/foo/bar/baz", "/foo/qux/quux"), "../../qux/quux".into()),
            (("/foo/bar", "/baz/qux"), "../../baz/qux".into()),
            (("/", "/foo"), "foo".into()),
            (("/foo", "/"), "..".into()),
            (("/", "/foo/bar"), "foo/bar".into()),
            (("/foo/bar", "/"), "../..".into()),
            (("/foo/bar/baz", "/foo/bar"), "..".into()),
            (("/foo/bar", "/foo/bar/baz/qux"), "baz/qux".into()),
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
