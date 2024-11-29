use std::env;

use super::{
    ParsedPath, PathInterface,
    _common::{
        constants::CHAR_FORWARD_SLASH,
        normalize_string::normalize_string,
        util::{char_code_at, js_slice},
    },
};

fn is_posix_path_separator(code: u32) -> bool {
    code == CHAR_FORWARD_SLASH
}

fn posix_cwd() -> String {
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

/// Posix implementation of the NodeJS path module
pub struct Posix;

impl PathInterface for Posix {
    fn sep() -> &'static str {
        "/"
    }

    /// Returns an object containing the parsed components of the path.
    ///
    /// This function is a faithful port of the Deno implementation of path parsing,
    /// adapted for Rust. Credit to the Deno project for the original implementation.
    ///
    /// # Examples
    ///
    /// ```
    /// use spellrs_url::path::parse;
    ///
    /// // Example for Windows
    /// #[cfg(windows)]
    /// {
    ///     let parsed_path = parse(r"C:\path\to\script.rs");
    ///     assert_eq!(parsed_path.root, r"C:\");
    ///     assert_eq!(parsed_path.dir, r"C:\path\to");
    ///     assert_eq!(parsed_path.base, "script.rs");
    ///     assert_eq!(parsed_path.ext, ".rs");
    ///     assert_eq!(parsed_path.name, "script");
    /// }
    ///
    /// // Example for Unix-like systems
    /// #[cfg(unix)]
    /// {
    ///     let parsed_path = parse("/path/to/dir/script.rs");
    ///     assert_eq!(parsed_path.root, "/");
    ///     assert_eq!(parsed_path.dir, "/path/to/dir");
    ///     assert_eq!(parsed_path.base, "script.rs");
    ///     assert_eq!(parsed_path.ext, ".rs");
    ///     assert_eq!(parsed_path.name, "script");
    /// }
    /// ```
    ///
    /// # Arguments
    ///
    /// * `path` - A path to process.
    ///
    /// # Returns
    ///
    /// An object with the parsed path components.
    ///
    /// # Attribution
    ///
    /// This implementation is a faithful port of the Deno project's `@std/path/parse` function.
    /// See [Deno](https://deno.land/) for the original implementation.
    fn parse(path: &str) -> ParsedPath {
        let mut ret = ParsedPath {
            dir: "".to_string(),
            root: "".to_string(),
            base: "".to_string(),
            name: "".to_string(),
            ext: "".to_string(),
        };

        if path.is_empty() {
            return ret;
        }

        let is_absolute = path.starts_with('/');

        if is_absolute {
            ret.root = "/".to_string();
        }

        let trimmed_path = path.trim_end_matches('/');
        let last_slash_index = trimmed_path.rfind('/');

        if let Some(index) = last_slash_index {
            if index > 0 || is_absolute {
                ret.dir = escape_special_chars(js_slice(trimmed_path, 0, index as isize));
            }

            ret.base = escape_special_chars(js_slice(
                trimmed_path,
                index as isize + 1,
                trimmed_path.len() as isize,
            ));
        } else {
            ret.base = escape_special_chars(trimmed_path.to_string());
        }

        if ret.base == ".." || ret.base == "." {
            ret.name = ret.base.clone();
        } else if let Some(dot_index) = ret.base.rfind('.') {
            if dot_index == 0 {
                ret.name = ret.base.clone();
            } else {
                ret.name = escape_special_chars(js_slice(&ret.base, 0, dot_index as isize));
                ret.ext = js_slice(&ret.base, dot_index as isize, ret.base.len() as isize);
            }
        } else {
            ret.name = ret.base.clone();
        }

        if ret.dir.is_empty() && is_absolute {
            ret.dir = "/".to_string();
        }

        ret
    }

    fn resolve(paths: &[&str]) -> String {
        if paths.len() == 1 && paths[0].is_empty() {
            return posix_cwd();
        }

        let mut resolved_path = "".to_string();
        let mut resolved_absolute = false;

        let mut i = paths.len() as isize - 1;
        while i >= 0 && !resolved_absolute {
            let path = paths[i as usize];
            if path.is_empty() {
                continue;
            }

            resolved_path = format!("{path}/{resolved_path}");
            resolved_absolute = char_code_at(path, 0) == CHAR_FORWARD_SLASH as i32;

            i -= 1;
        }

        if !resolved_absolute {
            let cwd = posix_cwd();
            resolved_path = format!("{cwd}/{resolved_path}");
            resolved_absolute = char_code_at(&cwd, 0) == CHAR_FORWARD_SLASH as i32;
        }

        resolved_path = normalize_string(
            &resolved_path,
            !resolved_absolute,
            '/',
            is_posix_path_separator,
        );

        if resolved_absolute {
            return format!("/{resolved_path}");
        }

        match !resolved_path.is_empty() {
            true => resolved_path,
            false => ".".to_string(),
        }
    }

    fn normalize(path: &str) -> String {
        if path.is_empty() {
            return ".".to_string();
        }

        let is_absolute = path.starts_with('/');
        let trailing_separator = path.ends_with('/');
        let mut path = normalize_string(path, !is_absolute, '/', is_posix_path_separator);

        if path.is_empty() {
            if is_absolute {
                return "/".to_string();
            }
            if trailing_separator {
                return "./".to_string();
            } else {
                return ".".to_string();
            }
        }

        if trailing_separator {
            path.push('/');
        }

        match is_absolute {
            true => format!("/{path}"),
            false => path,
        }
    }

    fn relative(from: &str, to: &str) -> String {
        if from == to {
            return "".to_string();
        }

        let from = Posix::resolve(&[from]);
        let to = Posix::resolve(&[to]);

        if from == to {
            return "".to_string();
        }

        let mut from_start = 1;
        let from_end = from.len();

        while from_start < from_end {
            if !is_posix_path_separator(char_code_at(&to, from_start) as u32) {
                break;
            }
            from_start += 1;
        }
        let from_len = from_end - from_start;

        // trim any leading backslashes
        let mut to_start = 1;
        let to_end = to.len();
        while to_start < to_end {
            if !is_posix_path_separator(char_code_at(&to, to_start) as u32) {
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
                    if is_posix_path_separator(char_code_at(&to, to_start + i) as u32) {
                        // We get here if `from` is the exact base path for `to`.
                        // For example: from='/foo/bar'; to='/foo/bar/baz'
                        return js_slice(&to, (to_start + i) as isize + 1, to.len() as isize);
                    } else if i == 0 {
                        // We get here if `from` is the root
                        // For example: from='/'; to='/foo'
                        return js_slice(&to, (to_start + i) as isize, to.len() as isize);
                    }
                } else if from_len > length {
                    if is_posix_path_separator(char_code_at(&from, from_start + i) as u32) {
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
            let from_code = char_code_at(&from, from_start + i);
            let to_code = char_code_at(&to, to_start + i);

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
            if i == from_end || is_posix_path_separator(char_code_at(&from, i) as u32) {
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
            let slice = js_slice(&to, start_idx as isize, to.len() as isize);
            return out + &slice;
        }

        to_start = match last_common_sep {
            Some(last_common_sep) => to_start + last_common_sep,
            None => to_start - 1,
        };

        if is_posix_path_separator(char_code_at(&to, to_start) as u32) {
            to_start += 1;
        }

        js_slice(&to, to_start as isize, to.len() as isize)
    }

    fn is_absolute(path: &str) -> bool {
        !path.is_empty() && path.starts_with('/')
    }
}

fn escape_special_chars(input: String) -> String {
    // handle null terminated bytes
    if input.contains(r"\0") {
        return input.replace(r"\0", r"\u0000");
    }

    if input.contains("\\") {
        return input.replace("\\", "\\\\");
    }

    input
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_posix_normalize() {
        let cases = vec![
            ("/path/to/some/../folder", "/path/to/folder"),
            ("./relative/path/./to/file", "relative/path/to/file"),
            ("C:\\folder\\..\\file.txt", "C:\\folder\\..\\file.txt"),
            ("/another/path/./to/normalize", "/another/path/to/normalize"),
            ("../outside/relative/path", "../outside/relative/path"),
            ("/root//double/slash/", "/root/double/slash/"),
            ("folder/with/extra/../..", "folder"),
            ("/final/example//path", "/final/example/path"),
        ];

        for (i, (path, expected)) in cases.iter().enumerate() {
            let result = Posix::normalize(path);
            assert_eq!(
                &result, expected,
                "\n\nPATH {i} FAILED\ninput:    {path}\nresult  : {:?}\nexpected: {:?}\n\n",
                result, expected
            );
        }
    }

    #[test]
    fn test_posix_parse() {
        let cases = get_parse_cases();

        for (i, (path, expected)) in cases.iter().enumerate() {
            let result = Posix::parse(path);
            assert_eq!(
                &result, expected,
                "\n\nPATH {i} FAILED\ninput:    {path}\nresult  : {:?}\nexpected: {:?}\n\n",
                result, expected
            );
        }
    }

    #[test]
    fn test_posix_resolve() {
        #[rustfmt::skip]
        let cases: Vec<(&[&str], &str)> = vec![
            (&["/absolute/path/one"], "/absolute/path/one"),
            (&["./relative/path/two"], "/home/roy/dev/rust/spellrs/spellrs_url/relative/path/two"),
            (&["../relative/parent/path/three"], "/home/roy/dev/rust/spellrs/relative/parent/path/three"),
            (&["/absolute/../path/four"], "/path/four"),
            (&["/absolute/path/five/.."], "/absolute/path"),
            (&["./relative/path/six/."], "/home/roy/dev/rust/spellrs/spellrs_url/relative/path/six"),
            (&["../../relative/parent/path/seven"], "/home/roy/dev/rust/relative/parent/path/seven"),
            (&["/absolute/path/eight/../../relative/path/nine"], "/absolute/relative/path/nine"),
            (&["/absolute/./path/ten"], "/absolute/path/ten"),
            (&["/absolute/path/eleven/../twelve"], "/absolute/path/twelve"),
            (&["relative/path/thirteen"], "/home/roy/dev/rust/spellrs/spellrs_url/relative/path/thirteen"),
            (&["relative/./path/fourteen"], "/home/roy/dev/rust/spellrs/spellrs_url/relative/path/fourteen"),
            (&["relative/../path/fifteen"], "/home/roy/dev/rust/spellrs/spellrs_url/path/fifteen"),
            (&["./"], "/home/roy/dev/rust/spellrs/spellrs_url"),
            (&["/"], "/"),
            (&["../"], "/home/roy/dev/rust/spellrs"),
            (&[""], "/home/roy/dev/rust/spellrs/spellrs_url"),
            (&["./relative", "../parent", "final/destination"], "/home/roy/dev/rust/spellrs/spellrs_url/parent/final/destination"),
            (&["/absolute", "./relative", "../parent", "final"], "/absolute/parent/final"),
            (&["/", "absolute/path", "to/resolve"], "/absolute/path/to/resolve"),
            (&["", "/absolute", "path/to/test"], "/absolute/path/to/test"),
            (&["relative", "/absolute", "./path/overwrite"], "/absolute/path/overwrite"),
            (&["../relative", "next", "/absolute/path"], "/absolute/path"),
            (&["../../relative", "./nested/dir", "final"], "/home/roy/dev/rust/relative/nested/dir/final"),
            (&["./", "../", "/absolute"], "/absolute"),
            (&["../", "./nested", "../../parent", "final"], "/home/roy/dev/rust/parent/final"),
            (&["/absolute", "/overwrite/absolute/path"], "/overwrite/absolute/path"),
            (&["/foo", "bar", "baz/asdf", "quux", ".."], "/foo/bar/baz/asdf")
        ];

        for (i, (paths, expected)) in cases.iter().enumerate() {
            let result = Posix::resolve(paths);
            assert_eq!(
                &result, expected,
                "\n\nCASE {i} FAILED\nPATHS    : \"{:?}\"\nresult  : {:?}\nexpected: {:?}\n\n",
                paths, result, expected
            );
        }
    }

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
            let result = Posix::relative(from, to);
            assert_eq!(
                &result, expected,
                "\n\nCASE {i} FAILED\nFROM    : \"{from}\"\nTO      : \"{to}\"\nresult  : {:?}\nexpected: {:?}\n\n",
                result, expected
            );
        }
    }

    fn get_parse_cases() -> Vec<(String, ParsedPath)> {
        vec![
            (
                r"a/b/c".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"a/b".into(),
                    base: r"c".into(),
                    ext: r"".into(),
                    name: r"c".into(),
                },
            ),
            (
                r"/a/b/c".into(),
                ParsedPath {
                    root: r"/".into(),
                    dir: r"/a/b".into(),
                    base: r"c".into(),
                    ext: r"".into(),
                    name: r"c".into(),
                },
            ),
            (
                r"a/b/../c".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"a/b/..".into(),
                    base: r"c".into(),
                    ext: r"".into(),
                    name: r"c".into(),
                },
            ),
            (
                r"a/b/./c".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"a/b/.".into(),
                    base: r"c".into(),
                    ext: r"".into(),
                    name: r"c".into(),
                },
            ),
            (
                r"/a/b/../../c".into(),
                ParsedPath {
                    root: r"/".into(),
                    dir: r"/a/b/../..".into(),
                    base: r"c".into(),
                    ext: r"".into(),
                    name: r"c".into(),
                },
            ),
            (
                r"/../a/b".into(),
                ParsedPath {
                    root: r"/".into(),
                    dir: r"/../a".into(),
                    base: r"b".into(),
                    ext: r"".into(),
                    name: r"b".into(),
                },
            ),
            (
                r"/a/b/../..".into(),
                ParsedPath {
                    root: r"/".into(),
                    dir: r"/a/b/..".into(),
                    base: r"..".into(),
                    ext: r"".into(),
                    name: r"..".into(),
                },
            ),
            (
                r"a/../../b".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"a/../..".into(),
                    base: r"b".into(),
                    ext: r"".into(),
                    name: r"b".into(),
                },
            ),
            (
                r"/.".into(),
                ParsedPath {
                    root: r"/".into(),
                    dir: r"/".into(),
                    base: r".".into(),
                    ext: r"".into(),
                    name: r".".into(),
                },
            ),
            (
                r".".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"".into(),
                    base: r".".into(),
                    ext: r"".into(),
                    name: r".".into(),
                },
            ),
            (
                r"./a/b".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"./a".into(),
                    base: r"b".into(),
                    ext: r"".into(),
                    name: r"b".into(),
                },
            ),
            (
                r"a/b/".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"a".into(),
                    base: r"b".into(),
                    ext: r"".into(),
                    name: r"b".into(),
                },
            ),
            (
                r"a/b//c".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"a/b/".into(),
                    base: r"c".into(),
                    ext: r"".into(),
                    name: r"c".into(),
                },
            ),
            (
                r"/".into(),
                ParsedPath {
                    root: r"/".into(),
                    dir: r"/".into(),
                    base: r"".into(),
                    ext: r"".into(),
                    name: r"".into(),
                },
            ),
            (
                r"/a".into(),
                ParsedPath {
                    root: r"/".into(),
                    dir: r"/".into(),
                    base: r"a".into(),
                    ext: r"".into(),
                    name: r"a".into(),
                },
            ),
            (
                r"/a/b".into(),
                ParsedPath {
                    root: r"/".into(),
                    dir: r"/a".into(),
                    base: r"b".into(),
                    ext: r"".into(),
                    name: r"b".into(),
                },
            ),
            (
                r"/a/b/.".into(),
                ParsedPath {
                    root: r"/".into(),
                    dir: r"/a/b".into(),
                    base: r".".into(),
                    ext: r"".into(),
                    name: r".".into(),
                },
            ),
            (
                r"/a/b/..".into(),
                ParsedPath {
                    root: r"/".into(),
                    dir: r"/a/b".into(),
                    base: r"..".into(),
                    ext: r"".into(),
                    name: r"..".into(),
                },
            ),
            (
                r"a".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"".into(),
                    base: r"a".into(),
                    ext: r"".into(),
                    name: r"a".into(),
                },
            ),
            (
                r"a/".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"".into(),
                    base: r"a".into(),
                    ext: r"".into(),
                    name: r"a".into(),
                },
            ),
            (
                r"a/b/c".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"a/b".into(),
                    base: r"c".into(),
                    ext: r"".into(),
                    name: r"c".into(),
                },
            ),
            (
                r"a/../b".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"a/..".into(),
                    base: r"b".into(),
                    ext: r"".into(),
                    name: r"b".into(),
                },
            ),
            (
                r"../a/b".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"../a".into(),
                    base: r"b".into(),
                    ext: r"".into(),
                    name: r"b".into(),
                },
            ),
            (
                r"./a/b".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"./a".into(),
                    base: r"b".into(),
                    ext: r"".into(),
                    name: r"b".into(),
                },
            ),
            (
                r"a/b/../../c".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"a/b/../..".into(),
                    base: r"c".into(),
                    ext: r"".into(),
                    name: r"c".into(),
                },
            ),
            (
                r"/a/b/c/file.txt".into(),
                ParsedPath {
                    root: r"/".into(),
                    dir: r"/a/b/c".into(),
                    base: r"file.txt".into(),
                    ext: r".txt".into(),
                    name: r"file".into(),
                },
            ),
            (
                r"a/b/c/file.txt".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"a/b/c".into(),
                    base: r"file.txt".into(),
                    ext: r".txt".into(),
                    name: r"file".into(),
                },
            ),
            (
                r"a/b/../file.txt".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"a/b/..".into(),
                    base: r"file.txt".into(),
                    ext: r".txt".into(),
                    name: r"file".into(),
                },
            ),
            (
                r"a/b/./file.txt".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"a/b/.".into(),
                    base: r"file.txt".into(),
                    ext: r".txt".into(),
                    name: r"file".into(),
                },
            ),
            (
                r"/a/b/../../file.txt".into(),
                ParsedPath {
                    root: r"/".into(),
                    dir: r"/a/b/../..".into(),
                    base: r"file.txt".into(),
                    ext: r".txt".into(),
                    name: r"file".into(),
                },
            ),
            (
                r"./file.txt".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r".".into(),
                    base: r"file.txt".into(),
                    ext: r".txt".into(),
                    name: r"file".into(),
                },
            ),
            (
                r"../file.txt".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"..".into(),
                    base: r"file.txt".into(),
                    ext: r".txt".into(),
                    name: r"file".into(),
                },
            ),
            (
                r"/file.txt".into(),
                ParsedPath {
                    root: r"/".into(),
                    dir: r"/".into(),
                    base: r"file.txt".into(),
                    ext: r".txt".into(),
                    name: r"file".into(),
                },
            ),
            (
                r"file.txt".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"".into(),
                    base: r"file.txt".into(),
                    ext: r".txt".into(),
                    name: r"file".into(),
                },
            ),
            (
                r"/a/b/c/.hiddenfile".into(),
                ParsedPath {
                    root: r"/".into(),
                    dir: r"/a/b/c".into(),
                    base: r".hiddenfile".into(),
                    ext: r"".into(),
                    name: r".hiddenfile".into(),
                },
            ),
            (
                r"a/b/c/.hiddenfile".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"a/b/c".into(),
                    base: r".hiddenfile".into(),
                    ext: r"".into(),
                    name: r".hiddenfile".into(),
                },
            ),
            (
                r"a/b/c/special-file.name".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"a/b/c".into(),
                    base: r"special-file.name".into(),
                    ext: r".name".into(),
                    name: r"special-file".into(),
                },
            ),
            (
                r"/a/b/c/file.with.many.dots.ext".into(),
                ParsedPath {
                    root: r"/".into(),
                    dir: r"/a/b/c".into(),
                    base: r"file.with.many.dots.ext".into(),
                    ext: r".ext".into(),
                    name: r"file.with.many.dots".into(),
                },
            ),
            (
                r"file-no-extension".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"".into(),
                    base: r"file-no-extension".into(),
                    ext: r"".into(),
                    name: r"file-no-extension".into(),
                },
            ),
            (
                r"/path/to/file-no-extension".into(),
                ParsedPath {
                    root: r"/".into(),
                    dir: r"/path/to".into(),
                    base: r"file-no-extension".into(),
                    ext: r"".into(),
                    name: r"file-no-extension".into(),
                },
            ),
            (
                r"a/b/..//c".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"a/b/../".into(),
                    base: r"c".into(),
                    ext: r"".into(),
                    name: r"c".into(),
                },
            ),
            (
                r"a/./b/./c".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"a/./b/.".into(),
                    base: r"c".into(),
                    ext: r"".into(),
                    name: r"c".into(),
                },
            ),
            (
                r"a b/c d/e f".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"a b/c d".into(),
                    base: r"e f".into(),
                    ext: r"".into(),
                    name: r"e f".into(),
                },
            ),
            (
                r"/a/b%20c/d".into(),
                ParsedPath {
                    root: r"/".into(),
                    dir: r"/a/b%20c".into(),
                    base: r"d".into(),
                    ext: r"".into(),
                    name: r"d".into(),
                },
            ),
            (
                r"a/b\c".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"a".into(),
                    base: r"b\\c".into(),
                    ext: r"".into(),
                    name: r"b\\c".into(),
                },
            ),
            (
                r"a/b/\0c".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"a/b".into(),
                    base: r"\u0000c".into(),
                    ext: r"".into(),
                    name: r"\u0000c".into(),
                },
            ),
            (
                r"a/b/c/file with spaces.txt".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"a/b/c".into(),
                    base: r"file with spaces.txt".into(),
                    ext: r".txt".into(),
                    name: r"file with spaces".into(),
                },
            ),
            (
                r"/path/to/special-file@.txt".into(),
                ParsedPath {
                    root: r"/".into(),
                    dir: r"/path/to".into(),
                    base: r"special-file@.txt".into(),
                    ext: r".txt".into(),
                    name: r"special-file@".into(),
                },
            ),
            (
                r"a/b/c/#file$.txt".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"a/b/c".into(),
                    base: r"#file$.txt".into(),
                    ext: r".txt".into(),
                    name: r"#file$".into(),
                },
            ),
            (
                r"".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"".into(),
                    base: r"".into(),
                    ext: r"".into(),
                    name: r"".into(),
                },
            ),
            (
                r".".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"".into(),
                    base: r".".into(),
                    ext: r"".into(),
                    name: r".".into(),
                },
            ),
            (
                r"..".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"".into(),
                    base: r"..".into(),
                    ext: r"".into(),
                    name: r"..".into(),
                },
            ),
            (
                r"./".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"".into(),
                    base: r".".into(),
                    ext: r"".into(),
                    name: r".".into(),
                },
            ),
            (
                r"../".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"".into(),
                    base: r"..".into(),
                    ext: r"".into(),
                    name: r"..".into(),
                },
            ),
            (
                r"./../".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r".".into(),
                    base: r"..".into(),
                    ext: r"".into(),
                    name: r"..".into(),
                },
            ),
            (
                r"/./".into(),
                ParsedPath {
                    root: r"/".into(),
                    dir: r"/".into(),
                    base: r".".into(),
                    ext: r"".into(),
                    name: r".".into(),
                },
            ),
            (
                r"a//b/c".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"a//b".into(),
                    base: r"c".into(),
                    ext: r"".into(),
                    name: r"c".into(),
                },
            ),
            (
                r"a/.././../b".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"a/.././..".into(),
                    base: r"b".into(),
                    ext: r"".into(),
                    name: r"b".into(),
                },
            ),
            (
                r"./a/b/c/file.txt".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"./a/b/c".into(),
                    base: r"file.txt".into(),
                    ext: r".txt".into(),
                    name: r"file".into(),
                },
            ),
            (
                r"../a/b/c/file.txt".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"../a/b/c".into(),
                    base: r"file.txt".into(),
                    ext: r".txt".into(),
                    name: r"file".into(),
                },
            ),
            (
                r"/absolute/path/file.js".into(),
                ParsedPath {
                    root: r"/".into(),
                    dir: r"/absolute/path".into(),
                    base: r"file.js".into(),
                    ext: r".js".into(),
                    name: r"file".into(),
                },
            ),
            (
                r"relative/path/file.js".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"relative/path".into(),
                    base: r"file.js".into(),
                    ext: r".js".into(),
                    name: r"file".into(),
                },
            ),
            (
                r"/absolute/../relative/file.js".into(),
                ParsedPath {
                    root: r"/".into(),
                    dir: r"/absolute/../relative".into(),
                    base: r"file.js".into(),
                    ext: r".js".into(),
                    name: r"file".into(),
                },
            ),
            (
                r"relative/../absolute/file.js".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"relative/../absolute".into(),
                    base: r"file.js".into(),
                    ext: r".js".into(),
                    name: r"file".into(),
                },
            ),
            (
                r"/a/b/c/file.json".into(),
                ParsedPath {
                    root: r"/".into(),
                    dir: r"/a/b/c".into(),
                    base: r"file.json".into(),
                    ext: r".json".into(),
                    name: r"file".into(),
                },
            ),
            (
                r"../a/b/c/file.json".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"../a/b/c".into(),
                    base: r"file.json".into(),
                    ext: r".json".into(),
                    name: r"file".into(),
                },
            ),
            (
                r"C:/a/b/c/file.txt".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"C:/a/b/c".into(),
                    base: r"file.txt".into(),
                    ext: r".txt".into(),
                    name: r"file".into(),
                },
            ),
            (
                r"C:/a/b/../file.txt".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"C:/a/b/..".into(),
                    base: r"file.txt".into(),
                    ext: r".txt".into(),
                    name: r"file".into(),
                },
            ),
            (
                r"C:/a/b/./file.txt".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"C:/a/b/.".into(),
                    base: r"file.txt".into(),
                    ext: r".txt".into(),
                    name: r"file".into(),
                },
            ),
            (
                r"C:/path/to/file-no-extension".into(),
                ParsedPath {
                    root: r"".into(),
                    dir: r"C:/path/to".into(),
                    base: r"file-no-extension".into(),
                    ext: r"".into(),
                    name: r"file-no-extension".into(),
                },
            ),
        ]
    }
}
