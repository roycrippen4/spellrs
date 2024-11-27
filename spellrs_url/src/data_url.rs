use crate::{basename_of_url_pathname, has_protocol, to_url};
use once_cell::sync::Lazy;
use regex::Regex;

static REG_MATCH_FILENAME: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"filename=([^;,]*)").expect("Invalid regex pattern"));

pub fn url_basename(url: impl Into<String>) -> String {
    fn guess_data_url_name(header: &str) -> String {
        if let Some(f_matches) = REG_MATCH_FILENAME.captures(header) {
            return f_matches.get(1).map(|m| m.as_str()).unwrap().to_string();
        }
        let re = Regex::new(r"\W").unwrap();

        let mime = header[1..].split(';').collect::<Vec<&str>>()[1].to_string();
        re.replace_all(&mime, ".").clone().to_string()
    }

    let url = to_url(url, None).unwrap();

    if url.scheme() == "data:" {
        let header = url.path()[1..].split(',').collect::<Vec<&str>>()[0];
        return guess_data_url_name(header);
    }

    basename_of_url_pathname(url.path()).to_string()
}

pub fn is_data_url(url: impl Into<String>) -> bool {
    has_protocol(url, "data:")
}
