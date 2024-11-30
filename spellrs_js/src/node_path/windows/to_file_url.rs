use regex::Regex;
use url::Url;

use crate::node_path::_common::to_file_url::encode_whitespace;

use super::{is_absolute, Res};

/// Converts a Windows path to a file URL.
pub(crate) fn to_file_url(path: &str) -> Res<Url> {
    if !is_absolute(path) {
        return Err(format!("Path must be absolute: received \"{}\"", path).into());
    }

    // Regex to extract hostname and pathname
    let re = Regex::new(r"^(?:[/\\]{2}([^/\\]+)[/\\]?.*?)?(.*)$")?;
    let captures = re.captures(path).ok_or("Failed to match path")?;

    let hostname = captures.get(1).map(|m| m.as_str());
    let pathname = captures
        .get(2)
        .ok_or("Failed to extract pathname")?
        .as_str();

    let mut url = Url::parse("file:///")?;
    url.set_path(&encode_whitespace(
        &pathname.replace('\\', "/").replace('%', "%25"),
    ));

    if let Some(host) = hostname {
        if host != "localhost" {
            url.set_host(Some(host))?;
            if url.host_str().is_none() {
                return Err(format!("Invalid hostname: \"{}\"", host).into());
            }
        }
    }

    Ok(url)
}

#[cfg(test)]
mod tests {
    use super::to_file_url;

    #[test]
    #[rustfmt::skip]
    fn test_windows_to_file_url() {
        assert_eq!(to_file_url("/home/foo").unwrap().to_string(), "file:///home/foo");
        assert_eq!(to_file_url("/home/ ").unwrap().to_string(), "file:///home/%20");
        assert_eq!(to_file_url("/home/%20").unwrap().to_string(), "file:///home/%2520");
        assert_eq!(to_file_url("/home\\foo").unwrap().to_string(), "file:///home/foo");
        assert!(to_file_url("foo").is_err());
        assert_eq!(to_file_url("C:/").unwrap().to_string(), "file:///C:/");
        assert_eq!(to_file_url("//localhost/home/foo").unwrap().to_string(), "file:///home/foo");
        assert_eq!(to_file_url("//127.0.0.1/home/foo").unwrap().to_string(), "file://127.0.0.1/home/foo");
        assert_eq!(to_file_url("//localhost/").unwrap().to_string(), "file:///");
        assert_eq!(to_file_url("//127.0.0.1/").unwrap().to_string(), "file://127.0.0.1/");
        assert!(to_file_url("//:/home/foo").is_err());
    }
}
