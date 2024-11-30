use crate::{ParsedPath, JS};

use super::util::escape_special_chars;

pub(crate) fn parse(path: &str) -> ParsedPath {
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
            ret.dir = escape_special_chars(trimmed_path.slice(0, index as isize));
        }

        ret.base = escape_special_chars(
            trimmed_path.slice(index as isize + 1, trimmed_path.len() as isize),
        );
    } else {
        ret.base = escape_special_chars(trimmed_path);
    }

    if ret.base == ".." || ret.base == "." {
        ret.name = ret.base.clone();
    } else if let Some(dot_index) = ret.base.rfind('.') {
        if dot_index == 0 {
            ret.name = ret.base.clone();
        } else {
            ret.name = escape_special_chars(&ret.base.slice(0, dot_index as isize));
            ret.ext = ret.base.slice(dot_index as isize, ret.base.len() as isize);
        }
    } else {
        ret.name = ret.base.clone();
    }

    if ret.dir.is_empty() && is_absolute {
        ret.dir = "/".to_string();
    }

    ret
}

#[cfg(test)]
mod test {
    use crate::ParsedPath;

    use super::parse;

    #[test]
    fn test_posix_parse() {
        let cases = get_parse_cases();

        for (i, (path, expected)) in cases.iter().enumerate() {
            let result = parse(path);
            assert_eq!(
                &result, expected,
                "\n\nPATH {i} FAILED\ninput:    {path}\nresult  : {:?}\nexpected: {:?}\n\n",
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
