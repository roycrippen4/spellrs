use crate::node_path::_common::normalize_string::normalize_string;

use super::util::is_posix_path_separator;

pub(crate) fn normalize(path: &str) -> String {
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

#[cfg(test)]
mod test {
    use super::normalize;

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
            let result = normalize(path);
            assert_eq!(
                &result, expected,
                "\n\nPATH {i} FAILED\ninput:    {path}\nresult  : {:?}\nexpected: {:?}\n\n",
                result, expected
            );
        }
    }
}
