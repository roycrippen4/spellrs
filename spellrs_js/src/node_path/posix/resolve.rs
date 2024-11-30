use crate::{
    node_path::_common::{constants::CHAR_FORWARD_SLASH, normalize_string::normalize_string},
    JS,
};

use super::util::{is_posix_path_separator, posix_cwd};

pub(crate) fn resolve(paths: &[&str]) -> String {
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
        resolved_absolute = path.char_code_at(0) == CHAR_FORWARD_SLASH as i32;

        i -= 1;
    }

    if !resolved_absolute {
        let cwd = posix_cwd();
        resolved_path = format!("{cwd}/{resolved_path}");
        resolved_absolute = cwd.char_code_at(0) == CHAR_FORWARD_SLASH as i32;
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
#[cfg(test)]
mod test {
    use super::resolve;

    #[test]
    fn test_posix_resolve() {
        #[rustfmt::skip]
        let cases: Vec<(&[&str], &str)> = vec![
            (&[], "/home/roy/dev/rust/spellrs/spellrs_js"),
            (&["/absolute/path/one"], "/absolute/path/one"),
            (&["./relative/path/two"], "/home/roy/dev/rust/spellrs/spellrs_js/relative/path/two"),
            (&["../relative/parent/path/three"], "/home/roy/dev/rust/spellrs/relative/parent/path/three"),
            (&["/absolute/../path/four"], "/path/four"),
            (&["/absolute/path/five/.."], "/absolute/path"),
            (&["./relative/path/six/."], "/home/roy/dev/rust/spellrs/spellrs_js/relative/path/six"),
            (&["../../relative/parent/path/seven"], "/home/roy/dev/rust/relative/parent/path/seven"),
            (&["/absolute/path/eight/../../relative/path/nine"], "/absolute/relative/path/nine"),
            (&["/absolute/./path/ten"], "/absolute/path/ten"),
            (&["/absolute/path/eleven/../twelve"], "/absolute/path/twelve"),
            (&["relative/path/thirteen"], "/home/roy/dev/rust/spellrs/spellrs_js/relative/path/thirteen"),
            (&["relative/./path/fourteen"], "/home/roy/dev/rust/spellrs/spellrs_js/relative/path/fourteen"),
            (&["relative/../path/fifteen"], "/home/roy/dev/rust/spellrs/spellrs_js/path/fifteen"),
            (&["./"], "/home/roy/dev/rust/spellrs/spellrs_js"),
            (&["/"], "/"),
            (&["../"], "/home/roy/dev/rust/spellrs"),
            (&[""], "/home/roy/dev/rust/spellrs/spellrs_js"),
            (&["./relative", "../parent", "final/destination"], "/home/roy/dev/rust/spellrs/spellrs_js/parent/final/destination"),
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
            let result = resolve(paths);
            assert_eq!(
                &result, expected,
                "\n\nCASE {i} FAILED\nPATHS    : \"{:?}\"\nresult  : {:?}\nexpected: {:?}\n\n",
                paths, result, expected
            );
        }
    }
}
