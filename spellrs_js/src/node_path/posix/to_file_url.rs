use url::Url;

use crate::node_path::_common::to_file_url::encode_whitespace;

use super::{is_absolute::is_absolute, Res};

pub fn to_file_url(path: &str) -> Res<Url> {
    if !is_absolute(path) {
        return Err(format!("Path must be absolute: received \"{}\"", path).into());
    }

    let encoded_path = encode_whitespace(&path.replace('%', "%25").replace('\\', "%5C"));

    let mut url = Url::parse("file:///")?;
    url.set_path(&encoded_path);
    Ok(url)
}

#[cfg(test)]
mod test {
    use super::to_file_url;

    #[test]
    #[rustfmt::skip]
    fn test_posix_to_file_url() {
        assert_eq!(to_file_url("/home/foo").unwrap().to_string(), "file:///home/foo");
        assert_eq!(to_file_url("/home/ ").unwrap().to_string(), "file:///home/%20");
        assert_eq!(to_file_url("/home/%20").unwrap().to_string(), "file:///home/%2520");
        assert_eq!(to_file_url("/home\\foo").unwrap().to_string(), "file:///home%5Cfoo");
        assert!(to_file_url("foo").is_err());
        assert!(to_file_url("C:/").is_err());
        assert_eq!(to_file_url("//localhost/home/foo").unwrap().to_string(), "file:///localhost/home/foo");
        assert_eq!(to_file_url("//localhost/").unwrap().to_string(), "file:///localhost/");
        assert_eq!(to_file_url("//:/home/foo").unwrap().to_string(), "file:///:/home/foo");
    }
}
