use std::any::Any;

use crate::traits::*;
use once_cell::sync::Lazy;
use regex::Regex;
use url::{ParseError, Url};

#[allow(unused)]
static IS_URL_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)^(\w[\w-]{1,63}:/|data:|stdin:)").unwrap());
pub static REGEX_WINDOWS_PATH: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?<path>^[\\/][a-zA-Z]:[\\/])").unwrap());
#[allow(unused)]
pub static REGEX_ENCODED_COLON: Lazy<Regex> = Lazy::new(|| Regex::new(r"%3[aA]").unwrap());

/// Try to make a URL, optionally resolving it relative to another URL.
/// If `relative_to` is provided, the `url` is treated as relative to it.
///
/// # Arguments
/// - `url`: The URL to parse, either as a string or `Url`.
/// - `relative_to`: Optional base URL to resolve relative URLs against.
///
/// # Returns
/// A normalized `Url`.
pub fn to_url(url: impl Into<String>, relative_to: Option<String>) -> Result<Url, ParseError> {
    normalize_windows_url(match relative_to {
        Some(base) => Url::parse(&base)?.join(&url.into())?,
        None => Url::parse(&url.into())?,
    })
}

pub fn url_parent(url: impl Into<String>) -> Result<Url, ParseError> {
    let url = to_url(url, None)?;
    if url.scheme() == "data" {
        return Ok(url);
    }

    let has_trailing_slash = url.path().ends_with('/');

    if !url.path().starts_with('/') {
        let parts: Vec<&str> = url.path().split("/").collect();
        let mut parent_path: String = match has_trailing_slash {
            true => parts[..parts.len().saturating_sub(2)].join("/"),
            false => parts[..parts.len().saturating_sub(1)].join("/"),
        };

        if !parent_path.is_empty() {
            parent_path.push('/');
        }

        let mut new_url = url.scheme().to_string();
        new_url.push(':');
        new_url.push_str(&parent_path);

        if let Some(query) = url.query() {
            new_url.push('?');
            new_url.push_str(query);
        }

        if let Some(fragment) = url.fragment() {
            new_url.push('#');
            new_url.push_str(fragment);
        }

        return Url::parse(&new_url);
    }

    let extra = if has_trailing_slash { ".." } else { "." };

    Url::parse(url.as_str())?.join(extra)
}

/// Returns the basename (last portion of the URL pathname) of a path.
/// It does NOT remove the trailing slash.
///
/// # Arguments
/// * `path` - URL pathname to extract the basename from.
///
/// # Examples
/// ```
/// use spellrs_url::basename_of_url_pathname;
///
/// let path = "https://example.com/some/path/";
/// assert_eq!(basename_of_url_pathname(path), "path/");
///
/// let path_no_slash = "https://example.com/some/path";
/// assert_eq!(basename_of_url_pathname(path_no_slash), "path");
/// ```
pub fn basename_of_url_pathname(path: &str) -> &str {
    if path == "/" || path.len() == 1 {
        return "";
    }

    let adj = if path.ends_with('/') { 2 } else { 0 };
    let idx = path[..path.len() - adj].rfind('/');

    match idx {
        Some(idx) => &path[idx + 1..],
        None => path,
    }
}

/// Determines if the provided value is URL-like.
///
/// This function leverages the `IsUrlLike` trait to check if a value
/// represents a URL or resembles one. It is generic over any type
/// that implements the `IsUrlLike` trait.
///
/// # Arguments
/// * `filename` - A value that implements the `IsUrlLike` trait.
///
/// # Examples
/// ```
/// use url::Url;
/// use spellrs_url::is_url_like;
///
/// let url = Url::parse("https://example.com").unwrap();
/// let string = "https://example.com".to_string();
/// let str = "https://example.com";
///
/// assert!(is_url_like(url));
/// assert!(is_url_like(string));
/// assert!(is_url_like(str));
/// ```
pub fn is_url_like(filename: impl IsUrlLike) -> bool {
    filename.is_url_like()
}

/// Checks if the given URL has the specified protocol.
///
/// This function verifies whether the given URL string or `Url` object has the specified protocol
/// (scheme). It automatically normalizes the protocol string to handle cases where it may or may not
/// include a trailing colon (`:`).
///
/// # Arguments
/// * `url` - The URL to check, provided as a `String`, `&str`, or any type that implements `Into<String>`.
/// * `scheme` - The protocol to check against, e.g., "file", "http", "https".
///
/// # Returns
/// `true` if the URL has the specified protocol, `false` otherwise.
///
/// # Examples
///
/// Basic usage:
/// ```
/// use spellrs_url::has_protocol;
///
/// assert!(has_protocol("https://example.com", "https")); // True: URL has "https" protocol
/// assert!(!has_protocol("https://example.com", "file")); // False: URL does not have "file" protocol
/// assert!(has_protocol("file://path/to/file", "file")); // True: URL has "file" protocol
/// ```
///
/// Handling URLs with or without trailing colons in the scheme:
/// ```
/// use spellrs_url::has_protocol;
///
/// assert!(has_protocol("https://example.com", "https:")); // True
/// assert!(has_protocol("https://example.com", "https")); // True
/// assert!(!has_protocol("https://example.com", "http")); // False
/// assert!(has_protocol("http://example.com", "http:")); // True
/// ```
///
/// Invalid URLs:
/// ```
/// use spellrs_url::has_protocol;
///
/// assert!(!has_protocol("invalid-url", "http")); // False: URL parsing fails
/// assert!(!has_protocol("", "http")); // False: Empty string cannot have a protocol
/// ```
///
/// Custom protocols:
/// ```
/// use spellrs_url::has_protocol;
///
/// assert!(has_protocol("custom-protocol://example", "custom-protocol")); // True
/// assert!(!has_protocol("custom-protocol://example", "https")); // False
/// ```
pub fn has_protocol(url: impl Into<String>, scheme: &str) -> bool {
    // Ensure the scheme ends with ':'
    let scheme = if scheme.ends_with(':') {
        scheme.to_string()
    } else {
        let mut new_scheme = scheme.to_string();
        new_scheme.push(':');
        new_scheme
    };

    // Parse the URL and check its scheme
    match Url::parse(&url.into()) {
        Ok(parsed_url) => parsed_url.scheme() == scheme.trim_end_matches(':'),
        Err(_) => false,
    }
}

/// Attempts to add a trailing slash to the Url pathname if it does not already have one.
/// If the pathname doesn't start with a `/`, a trailing slash is _not_ added.
///
/// # Arguments
/// * `url` - The URL to modify.
///
/// # Returns
/// The original URL if a trailing slash cannot be added, or a new URL with the trailing slash added.
///
/// # Examples
///
/// Basic usage:
/// ```
/// use spellrs_url::add_trailing_slash;
/// use url::Url;
///
/// let url = Url::parse("https://example.com/path").unwrap();
/// let modified_url = add_trailing_slash(url);
/// assert_eq!(modified_url.as_str(), "https://example.com/path/");
/// ```
///
/// Pathnames already ending with `/`:
/// ```
/// use spellrs_url::add_trailing_slash;
/// use url::Url;
///
/// let url = Url::parse("https://example.com/path/").unwrap();
/// let modified_url = add_trailing_slash(url);
/// assert_eq!(modified_url.as_str(), "https://example.com/path/");
/// ```
///
/// Pathnames that do not start with `/`:
/// ```
/// use spellrs_url::add_trailing_slash;
/// use url::Url;
///
/// let url = Url::parse("https://example.com").unwrap();
/// let modified_url = add_trailing_slash(url);
/// assert_eq!(modified_url.as_str(), "https://example.com/");
/// ```
///
/// Pathnames with no path at all:
/// ```
/// use spellrs_url::add_trailing_slash;
/// use url::Url;
///
/// let url = Url::parse("https://example.com").unwrap();
/// let modified_url = add_trailing_slash(url);
/// assert_eq!(modified_url.as_str(), "https://example.com/");
/// ```
pub fn add_trailing_slash(url: Url) -> Url {
    if url.path().ends_with('/') || !url.path().starts_with('/') {
        return url;
    }

    let mut path = url.path().to_string();
    let mut new_url = url.clone();
    path.push('/');
    new_url.set_path(&path);
    new_url
}

/// Remove the filename at the end of the URL path.
///
/// If the Url path ends with a `/`, it is considered a directory and the Url is returned as is.
/// If the Url path does not start with a `/`, it is considered a non-regular Url and it is
/// returned as is.
#[allow(unused)]
pub fn url_remove_filename(url: &Url) -> Result<Url, ParseError> {
    if url.path().ends_with('/') || !url.path().starts_with('/') {
        return Ok(url.clone());
    }
    url.join("./")
}

/// Extract the filename from the Url path.
#[allow(unused)]
pub fn url_filename(url: &Url) -> &str {
    if !url.path().starts_with('/') {
        return "";
    }
    match url.path().rfind('/') {
        Some(idx) => &url.path()[idx + 1..],
        None => "",
    }
}

/// Determines if the provided value is NOT URL-like.
///
/// This function is the negation of `is_url_like` and checks if a value
/// does not represent a URL or resemble one.
///
/// # Arguments
/// * `filename` - A value that implements the `IsUrlLike` trait.
///
/// # Examples
/// ```
/// use url::Url;
/// use spellrs_url::is_not_url_like;
///
/// let url = Url::parse("https://example.com").unwrap();
/// let string = "invalid-url".to_string();
///
/// assert!(!is_not_url_like(url));
/// assert!(is_not_url_like(string));
/// ```
///
/// # Notes
/// Ensure that you import the `IsUrlLike` trait:
/// ```
/// use spellrs_url::traits::*;
/// ```
pub fn is_not_url_like(filename: impl IsUrlLike) -> bool {
    !filename.is_url_like()
}

/// Checks if the given value is a `Url` instance.
///
/// # Arguments
/// * `value` - A reference to a value of unknown type.
///
/// # Returns
/// `true` if the value is a `Url`, `false` otherwise.
///
/// # Examples
/// ```
/// use url::Url;
/// use spellrs_url::is_url;
///
/// let url = Url::parse("https://example.com").unwrap();
/// assert!(is_url(&url));
///
/// let not_url = "https://example.com";
/// assert!(!is_url(&not_url));
/// ```
pub fn is_url(value: &dyn Any) -> bool {
    value.is::<Url>()
}

pub fn url_dirname(url: impl Into<String>) -> Result<Url, ParseError> {
    url_parent(url)
}

pub fn url_relative(from: impl Into<String>, to: impl Into<String>) -> Result<String, ParseError> {
    let from = &to_url(from, None)?;
    let to = &to_url(to, None)?;
    Ok(url_to_url_relative(from, to))
}

/// Calculate the relative path to go from `url_from` to `url_to`.
/// The protocol is not evaluated. Only the `url.path()` is used.
pub fn url_to_url_relative(url_from: &Url, url_to: &Url) -> String {
    let mut p_from = url_from.path().to_string();
    let p_to = url_to.path().to_string();

    if p_from == p_to {
        return String::new();
    }

    if !p_from.ends_with('/') {
        if let Ok(base_url) = url_from.join("./") {
            p_from = base_url.path().to_string()
        }
    }

    if p_to.starts_with(&p_from) {
        return decode_uri_component(&p_to[p_from.len()..]);
    }

    let is_empty = |s: &&str| !s.is_empty();
    let p_from_parts: Vec<&str> = p_from.split('/').filter(is_empty).collect();
    let p_to_parts: Vec<&str> = p_to.split('/').filter(is_empty).collect();

    let mut i = 0;
    while i < p_from_parts.len() && i < p_to_parts.len() && p_from_parts[i] == p_to_parts[i] {
        i += 1;
    }

    let up_levels = "../".repeat(p_from_parts.len() - i + 1);
    let remaining_path: String = p_to_parts[i - 1..].join("/");
    let rel_path = format!("{}{}", up_levels, remaining_path);
    let decoded_rel = decode_uri_component(&rel_path);
    let decoded_abs = decode_uri_component(&p_to);

    if decoded_rel.len() < decoded_abs.len() {
        decoded_rel
    } else {
        decoded_abs
    }
}

/// Ensure that a windows file url is correctly formatted with a capitol letter for the drive
pub fn normalize_windows_url(input: impl Into<String>) -> Result<Url, url::ParseError> {
    let mut url = Url::parse(&input.into())?;

    if url.scheme() == "file" {
        let decoded_path = url.path().replace("%3A", ":").replace("%3a", ":");
        let re = Regex::new(r"^/([a-z]):").unwrap();
        let updated_path = re.replace(&decoded_path, |caps: &regex::Captures| {
            format!("/{}:", &caps[1].to_uppercase())
        });

        if updated_path != url.path() {
            url.set_path(&updated_path);
        }
    }

    Ok(url)
}

pub fn decode_uri_component(input: &str) -> String {
    let mut decoded = String::new();
    let mut chars = input.chars();

    while let Some(c) = chars.next() {
        if c == '%' {
            let hex: String = chars.by_ref().take(2).collect();
            if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                decoded.push(byte as char);
            } else {
                decoded.push('%');
                decoded.push_str(&hex);
            }
        } else {
            decoded.push(c);
        }
    }

    decoded
}

#[cfg(test)]
mod test {
    use crate::url::normalize_windows_url;
    use url::Url;

    use super::*;

    #[test]
    fn test_is_url_like() {
        let cases = vec![
            ("samples/cities.txt", false),
            ("samples/cities.txt.gz", false),
            ("https://github.com/streetsidesoftware/cspell/raw/main/packages/cspell-io/samples/cities.txt", true),
            ("https://github.com/streetsidesoftware/cspell/raw/main/packages/cspell-io/samples/cities.txt.gz", true),
            ("vsls:/cspell.config.yaml", true),
            ("file:///", true),
            ("file:///samples/cities.txt", true),
            ("file:///samples/code/", true),
            ("stdin:sample.py", true),
            ("data:application/text", true),
            ("https://github.com/streetsidesoftware/samples/cities.txt", true),
            ("vs-code:///remote/file/sample.ts", true),
        ];

        for (file, expected) in cases {
            let result = is_url_like(file);
            assert_eq!(
                expected, result,
                "expected result {result} to be {expected}"
            )
        }
    }

    #[test]
    fn test_to_url() {
        let cases = vec![
            (
                "https://github.com/streetsidesoftware/cspell/README.md",
                None,
                "https://github.com/streetsidesoftware/cspell/README.md",
            ),
            (
                "README.md",
                Some("https://github.com/streetsidesoftware/cspell/".to_string()),
                "https://github.com/streetsidesoftware/cspell/README.md",
            ),
            ("vsls:/cspell.config.yaml", None, "vsls:/cspell.config.yaml"),
            (
                "stdin:sample.py",
                Some("file:///".to_string()),
                "stdin:sample.py",
            ),
            (
                "vsls:/cspell.config.yaml",
                Some("file:///".to_string()),
                "vsls:/cspell.config.yaml",
            ),
            (
                "**/*.json",
                Some("file:///User/test/project/".to_string()),
                "file:///User/test/project/**/*.json",
            ),
            (
                "**/*{.json,.jsonc,.yml}",
                Some("file:///User/test/project/".to_string()),
                "file:///User/test/project/**/*%7B.json,.jsonc,.yml%7D",
            ),
        ];

        for (url, root, expected) in cases {
            let result = to_url(url, root);
            let expected = Url::parse(expected);
            assert_eq!(expected, result);
        }

        assert_eq!(
            to_url(
                Url::parse("https://github.com/streetsidesoftware/cspell/README.md").unwrap(),
                None,
            ),
            Url::parse("https://github.com/streetsidesoftware/cspell/README.md")
        )
    }

    #[test]
    fn test_basename_of_url_pathname() {
        let cases = vec![
            ("/", ""),
            ("samples/cities.txt", "cities.txt"),
            ("samples/cities.txt.gz", "cities.txt.gz"),
            ("samples/code/", "code/"),
            ("file://samples/code/", "code/"),
            ("https://github.com/streetsidesoftware/cspell/raw/main/packages/cspell-io/samples/cities.txt", "cities.txt"),
            ("https://github.com/streetsidesoftware/cspell/raw/main/packages/cspell-io/samples/cities.txt.gz", "cities.txt.gz"),
            ("https://github.com/streetsidesoftware/cspell/raw/main/packages/cspell-io/samples/code/", "code/"),
        ];

        for (file, expected) in cases {
            let result = basename_of_url_pathname(file);
            assert_eq!(expected, result);
        }
    }

    #[test]
    fn test_url_parent() {
        let cases = vec![
            ("file:///", "file:///"),
            ("file:///samples/cities.txt", "file:///samples/"),
            ("file:///samples/code/", "file:///samples/"),
            (
                "https://github.com/streetsidesoftware/samples/cities.txt",
                "https://github.com/streetsidesoftware/samples/",
            ),
            (
                "stdin:/github.com/streetsidesoftware/samples/",
                "stdin:/github.com/streetsidesoftware/",
            ),
            (
                "stdin:github.com/streetsidesoftware/samples/",
                "stdin:github.com/streetsidesoftware/",
            ),
            ("vsls:/cspell.config.yaml", "vsls:/"),
            ("vsls:/path/file.txt", "vsls:/path/"),
        ];

        for (url, expected) in cases {
            let result = url_parent(url).unwrap().to_string();
            let expected = Url::parse(expected).unwrap().to_string();
            assert_eq!(expected, result)
        }
    }

    #[test]
    fn test_add_trailing_slash() {
        let cases = vec![
            ("file:///", "file:///"),
            ("file:///samples/code/", "file:///samples/code/"),
            ("file:///samples/code", "file:///samples/code/"),
            ("stdin:sample", "stdin:sample"),
            ("stdin:/sample", "stdin:/sample/"),
            ("data:application/text", "data:application/text"),
            (
                "https://github.com/streetsidesoftware/samples",
                "https://github.com/streetsidesoftware/samples/",
            ),
            (
                "vs-code:///remote/file/sample.ts",
                "vs-code:///remote/file/sample.ts/",
            ),
        ];

        for (url, expected) in cases {
            let expected = Url::parse(expected).unwrap();
            let result = add_trailing_slash(to_url(url, None).unwrap());
            assert_eq!(expected, result);
        }
    }

    #[test]
    fn test_url_relative() {
        let cases = vec![
            ("file:///", "file:///", ""),
            (
                "file:///samples/code/",
                "file:///samples/code/src/file.cpp",
                "src/file.cpp",
            ),
            (
                "file:///samples/code/package.json",
                "file:///samples/code/src/file.cpp",
                "src/file.cpp",
            ),
            ("file:///samples/code/", "file:///samples/code/", ""),
            ("file:///samples/code/", "file:///samples/code", "../code"),
            ("file:///samples/code", "file:///samples/code/", "code/"),
            ("stdin:sample", "stdin:sample", ""),
            ("stdin:/sample", "stdin:/sample", ""),
            ("data:application/text", "data:application/text", ""),
            (
                "https://github.com/streetsidesoftware/samples",
                "https://github.com/streetsidesoftware/samples",
                "",
            ),
            (
                "vs-code:///remote/file/sample.ts",
                "vs-code:///remote/file/sample.ts",
                "",
            ),
        ];

        for (from, to, expected) in cases {
            assert_eq!(&url_relative(from, to).unwrap(), expected);

            let rel = url_relative(to_url(from, None).unwrap(), to_url(to, None).unwrap()).unwrap();
            assert_eq!(rel, expected);

            if to_url(from, None).unwrap().path().starts_with('/') {
                let result = Url::parse(from).unwrap().join(&rel).unwrap();
                let expected = Url::parse(to).unwrap();
                assert_eq!(expected, result)
            }
        }
    }

    #[test]
    fn test_url_filename() {
        let cases = vec![
            ("file:///path/to/my/file.txt", "file.txt"),
            ("stdin:sample", ""),
        ];

        for (url, expected) in cases {
            let url = &Url::parse(url).unwrap();
            let result = url_filename(url);
            assert_eq!(expected, result);
        }
    }

    #[test]
    fn test_url_remove_filename() {
        let path = "file:///path/to/my/file.txt";
        let expected = "file.txt";

        let url = Url::parse(path).unwrap();
        assert_eq!(url_filename(&url), expected);

        let result = url_remove_filename(&url)
            .unwrap()
            .join(url_filename(&url))
            .unwrap();

        assert_eq!(url.as_str(), result.as_str());
    }

    #[test]
    fn test_normalize_windows_url() {
        let cases = vec![
            ("file:///path/to/my/file.txt", "file:///path/to/my/file.txt"),
            (
                "file:///C:/path/to/my/file.txt",
                "file:///C:/path/to/my/file.txt",
            ),
            (
                "file:///C%3a/path/to/my/file.txt",
                "file:///C:/path/to/my/file.txt",
            ),
            (
                "file:///d:/path/to/my/file.txt",
                "file:///D:/path/to/my/file.txt",
            ),
            (
                "file:///d%3a/path/to/my/file.txt",
                "file:///D:/path/to/my/file.txt",
            ),
            (
                "file:///d%3A/path/to/my/file.txt",
                "file:///D:/path/to/my/file.txt",
            ),
        ];

        for (url, expected) in cases {
            let url = Url::parse(url).unwrap();
            assert_eq!(normalize_windows_url(url).unwrap().as_str(), expected);
        }
    }
}
