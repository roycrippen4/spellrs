use crate::{
    node_path::_common::{
        constants::{CHAR_COLON, CHAR_FORWARD_SLASH},
        normalize_string::normalize_string,
    },
    JS,
};

use super::util::{is_path_separator, is_windows_device_root};

pub(crate) fn normalize(path: &str) -> String {
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

#[cfg(test)]
mod test {
    use super::normalize;

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
            let result = normalize(path);
            assert_eq!(
                &result, expected,
                "\n\nPATH {i} FAILED\ninput:    {path}\nresult  : {:?}\nexpected: {:?}\n\n",
                result, expected
            );
        }
    }
}
