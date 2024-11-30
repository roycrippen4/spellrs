use std::env;

use crate::{
    node_path::_common::{constants::CHAR_COLON, normalize_string::normalize_string},
    JS,
};

use super::util::{is_path_separator, is_windows_device_root};

pub fn resolve(paths: &[&str]) -> String {
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
        } else if is_windows_device_root(code as u32) && path.char_code_at(1) as u32 == CHAR_COLON {
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

    resolved_tail = normalize_string(&resolved_tail, !resolved_absolute, '\\', is_path_separator);

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
#[cfg(test)]
mod test {
    use super::resolve;

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
            let result = resolve(paths);
            assert_eq!(
                &result, expected,
                "\n\nCASE {i} FAILED\nPATHS    : \"{:?}\"\nresult  : {:?}\nexpected: {:?}\n\n",
                paths, result, expected
            );
        }
    }
}
