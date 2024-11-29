use std::env;

use super::{
    ParsedPath, PathInterface,
    _common::{
        constants::{
            CHAR_BACKWARD_SLASH, CHAR_COLON, CHAR_DOT, CHAR_FORWARD_SLASH, CHAR_LOWERCASE_A,
            CHAR_LOWERCASE_Z, CHAR_UPPERCASE_A, CHAR_UPPERCASE_Z,
        },
        normalize_string::normalize_string,
    },
};

use crate::JS;

fn is_path_separator(code: u32) -> bool {
    code == CHAR_FORWARD_SLASH || code == CHAR_BACKWARD_SLASH
}

fn is_windows_device_root(code: u32) -> bool {
    (CHAR_LOWERCASE_A..=CHAR_LOWERCASE_Z).contains(&code)
        || (CHAR_UPPERCASE_A..=CHAR_UPPERCASE_Z).contains(&code)
}

/// Windows implementation of the NodeJS path module
#[derive(Debug)]
pub struct Windows;

impl PathInterface for Windows {
    fn sep(&self) -> &'static str {
        r"\"
    }

    fn parse(&self, path: &str) -> ParsedPath {
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
            } else if is_windows_device_root(code as u32)
                && path.char_code_at(1) as u32 == CHAR_COLON
            {
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

    fn resolve(&self, paths: &[&str]) -> String {
        let mut resolved_device = "".to_string();
        let mut resolved_tail = "".to_string();
        let mut resolved_absolute = false;

        let mut i = paths.len() as isize - 1;
        while i >= -1 {
            #[allow(unused_assignments)]
            let mut path = "".to_string();

            if i >= 0 {
                path = paths[i as usize].to_string();

                if path.is_empty() {
                    continue;
                }
            } else if resolved_device.is_empty() {
                path = env::current_dir()
                    .expect("failed to get current directory")
                    .to_string_lossy()
                    .to_string();
            } else {
                let key = format!("={}", resolved_device);
                path = env::var(&key).unwrap_or_else(|_| {
                    env::current_dir()
                        .expect("Failed to get current directory")
                        .to_string_lossy()
                        .to_string()
                });

                if path.is_empty()
                    || path[0..2].to_lowercase() != resolved_device.to_lowercase()
                    || (path.chars().nth(2) == Some('\\'))
                {
                    path = format!("{}\\", resolved_device)
                }
            }

            let len = path.len();
            let mut root_end = 0;
            let mut device = "".to_string();
            let mut is_absolute = false;
            let code = path.char_code_at(0);

            if len == 1 {
                if is_path_separator(code as u32) {
                    root_end = 1;
                    is_absolute = true;
                }
            } else if is_path_separator(code as u32) {
                is_absolute = true;

                if is_path_separator(path.char_code_at(1) as u32) {
                    let mut j = 2;
                    let mut last = j;

                    while j < len && !is_path_separator(path.char_code_at(j) as u32) {
                        j += 1;
                    }

                    if j < len && j != last {
                        let first_part = path.slice(last as isize, j as isize);
                        last = j;

                        while j < len && is_path_separator(path.char_code_at(j) as u32) {
                            j += 1;
                        }

                        if j < len && j != last {
                            last = j;

                            while j < len && !is_path_separator(path.char_code_at(j) as u32) {
                                j += 1;
                            }
                            if j == len || j != last {
                                if first_part != "." && first_part != "?" {
                                    device = format!(
                                        "\\\\{first_part}\\{}",
                                        path.slice(last as isize, j as isize)
                                    );
                                    root_end = j;
                                } else {
                                    device = format!("\\\\{first_part}");
                                    root_end = 4;
                                }
                            }
                        }
                    }
                } else {
                    root_end = 1;
                }
            } else if is_windows_device_root(code as u32)
                && path.char_code_at(1) as u32 == CHAR_COLON
            {
                device = path.slice(0, 2);
                root_end = 2;

                if len > 2 && is_path_separator(path.char_code_at(2) as u32) {
                    is_absolute = true;
                    root_end = 2;
                }
            }

            if !device.is_empty() {
                if !resolved_device.is_empty() {
                    if device.to_lowercase() != resolved_device.to_lowercase() {
                        continue;
                    }
                } else {
                    resolved_device = device;
                }
            }

            if resolved_absolute {
                if !resolved_device.is_empty() {
                    break;
                }
            } else {
                let slice = path.slice(root_end as isize, path.len() as isize);
                resolved_tail = format!("{slice}\\{resolved_tail}");
                resolved_absolute = is_absolute;
                if is_absolute && !resolved_device.is_empty() {
                    break;
                }
            }
            i -= 1;
        }

        resolved_tail =
            normalize_string(&resolved_tail, !resolved_absolute, '\\', is_path_separator);

        match resolved_absolute {
            true => format!("{resolved_device}\\{resolved_tail}"),
            false => {
                let resolved = format!("{resolved_device}{resolved_tail}");
                match resolved.is_empty() {
                    true => ".".to_string(),
                    false => resolved,
                }
            }
        }
    }

    fn normalize(&self, path: &str) -> String {
        let len = path.len();
        if len == 0 {
            return ".".to_string();
        }
        let mut root_end = 0;
        let mut device: Option<String> = None;
        let mut is_absolute = false;
        let code = path.char_code_at(0);

        if len == 1 {
            return if code == CHAR_FORWARD_SLASH as i32 {
                "\\".to_string()
            } else {
                path.to_string()
            };
        }

        if is_path_separator(code as u32) {
            is_absolute = true;

            if is_path_separator(path.char_code_at(1) as u32) {
                let mut j = 2;
                let mut last = j;

                while j < len && !is_path_separator(path.char_code_at(j) as u32) {
                    j += 1;
                }
                if j < len && j != last {
                    let first_part = path.slice(last as isize, j as isize);
                    last = j;

                    while j < len && is_path_separator(path.char_code_at(j) as u32) {
                        j += 1;
                    }

                    if j < len && j != last {
                        last = j;

                        while j < len && !is_path_separator(path.char_code_at(j) as u32) {
                            j += 1;
                        }
                        if j == len || j != last {
                            if first_part == "." || first_part == "?" {
                                device = Some(format!("\\\\{first_part}"));
                                root_end = 4;
                            } else if j == len {
                                let slice = path.slice(last as isize, path.len() as isize);
                                return format!("\\\\{first_part}\\{slice}");
                            } else {
                                let slice = path.slice(last as isize, j as isize);
                                device = Some(format!("\\\\{first_part}\\{slice}"));
                                root_end = j;
                            }
                        }
                    }
                }
            } else {
                root_end = 1;
            }
        } else if is_windows_device_root(code as u32) && path.char_code_at(1) as u32 == CHAR_COLON {
            device = Some(path.slice(0, 2).to_string());
            root_end = 2;

            if len > 2 && is_path_separator(path.char_code_at(2) as u32) {
                is_absolute = true;
                root_end = 3;
            }
        }

        let mut tail = match root_end < len {
            true => normalize_string(
                path.slice(root_end as isize, path.len() as isize),
                !is_absolute,
                '\\',
                is_path_separator,
            ),
            false => "".to_string(),
        };

        if tail.is_empty() && !is_absolute {
            tail = ".".to_string()
        }

        if tail.is_empty() && is_path_separator(path.char_code_at(len - 1) as u32) {
            tail += "\\";
        }

        if device.is_none() {
            return match is_absolute {
                true => format!("\\{tail}"),
                false => tail,
            };
        }

        match is_absolute {
            true => format!("{}\\{tail}", device.unwrap()),
            false => format!("{}{tail}", device.unwrap()),
        }
    }

    fn relative(&self, from: &str, to: &str) -> String {
        let from_orig = Windows.resolve(&[from]);
        let to_orig = Windows.resolve(&[to]);

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

    fn is_absolute(&self, path: &str) -> bool {
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_windows_is_absolute() {
        assert!(Windows.is_absolute("C:\\foo\\bar"));
        assert!(!Windows.is_absolute("..\\baz"));
    }

    #[test]
    fn test_windows_normalize() {
        #[rustfmt::skip]
        let cases = vec![
            ("C:\\foo\\..\\bar", "C:\\bar"),
            ("/path/to/some/../folder", "\\path\\to\\folder"),
            ("./relative/path/./to/file", "relative\\path\\to\\file"),
            ("C:\\folder\\..\\file.txt", "C:\\file.txt"),
            ("/another/path/./to/normalize", "\\another\\path\\to\\normalize"),
            ("../outside/relative/path", "..\\outside\\relative\\path"),
            ("/root//double/slash/", "\\root\\double\\slash"),
            ("folder/with/extra/../..", "folder"),
            ("/final/example//path", "\\final\\example\\path"),
        ];

        for (i, (path, expected)) in cases.iter().enumerate() {
            let result = Windows.normalize(path);
            assert_eq!(
                &result, expected,
                "\n\nPATH {i} FAILED\ninput:    {path}\nresult  : {:?}\nexpected: {:?}\n\n",
                result, expected
            );
        }
    }

    #[test]
    fn test_windows_parse() {
        let parsed = Windows.parse("\\\\server\\share");
        let expected = ParsedPath {
            base: "\\".into(),
            dir: "\\\\server\\share".into(),
            ext: "".into(),
            name: "".into(),
            root: "\\\\server\\share".into(),
        };

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_windows_resolve() {
        #[rustfmt::skip]
        let test_cases: Vec<(&[&str], String)> = vec![
            (&["C:\\Users", "Documents", "file.txt"], String::from(r"C:\Users\Documents\file.txt")),
            (&["folder", "subfolder", "file.txt"], String::from(r"\home\roy\dev\rust\spellrs\spellrs_js\folder\subfolder\file.txt")),
            (&["C:\\", "folder", "file.txt"], String::from(r"C:\folder\file.txt")),
            (&["C:\\Users", "\\absolute\\path"], String::from(r"C:\absolute\path")),
            (&["\\\\server\\share", "folder", "file.txt"], String::from(r"\\server\share\folder\file.txt")),
            (&["folder", "..", "file.txt"], String::from(r"\home\roy\dev\rust\spellrs\spellrs_js\file.txt")),
            (&["C:\\folder\\subfolder", "..\\file.txt"], String::from(r"C:\folder\file.txt")),
            (&["C:\\Users", "C:\\Other\\Path"], String::from(r"C:\Other\Path")),
            (&["\\absolute\\path", "relative\\file"], String::from(r"\absolute\path\relative\file")),
        ];

        for (i, (paths, expected)) in test_cases.iter().enumerate() {
            let result = Windows.resolve(paths);
            assert_eq!(
                &result, expected,
                "\n\nCASE {i} FAILED\nPATHS    : \"{:?}\"\nresult  : {:?}\nexpected: {:?}\n\n",
                paths, result, expected
            );
        }
    }

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
            let result = Windows.relative(from, to);
            assert_eq!(
                &result, expected,
                "\n\nCASE {i} FAILED\nFROM    : \"{from}\"\nTO      : \"{to}\"\nresult  : {:?}\nexpected: {:?}\n\n",
                result, expected
            );
        }
    }
}
