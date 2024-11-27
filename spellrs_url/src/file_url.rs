use std::path::{Path, PathBuf};

use once_cell::sync::Lazy;
use regex::Regex;
use url::Url;

use crate::{decode_uri_component, has_protocol, StUrl};

/// Returns true if the Url is a file Url.
pub fn is_file_url(url: &StUrl) -> bool {
    has_protocol(url, "file:")
}

pub fn to_file_path_or_href(url: &StUrl) -> String {
    match is_file_url(url) {
        true => to_file_path(url),
        false => url.to_string(),
    }
}

fn to_file_path(url: &StUrl) -> String {
    let url_path = file_url_to_path(url.as_str()).unwrap();
    path_windows_drive_letter_to_upper(&url_path.as_os_str().to_string_lossy())
}

/// Converts a `file://` URL into a local file system path.
pub fn file_url_to_path(file_url: &str) -> Result<PathBuf, String> {
    // Parse the URL
    let url = Url::parse(file_url).map_err(|e| format!("Invalid URL: {}", e))?;

    // Ensure it uses the `file` scheme
    if url.scheme() != "file" {
        return Err("URL must use the 'file' scheme".to_string());
    }

    // Extract the path component and decode percent-encoded characters
    let path = url.path();
    let decoded_path = decode_uri_component(path);

    // On Windows, handle drive letters in the host component
    if cfg!(windows) {
        // Handle the case of a drive letter in the host
        let host = url.host_str().unwrap_or("");
        if !host.is_empty() {
            // Combine the host and path to create the Windows path
            return Ok(Path::new(&format!("{}:{}", host, decoded_path)).to_path_buf());
        }
    }

    // For non-Windows systems, or if no host, just return the decoded path
    Ok(Path::new(&decoded_path).to_path_buf())
}

// Define a static lazy regex
pub static RE_WINDOWS_PATH_DRIVE_LETTER: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^([a-zA-Z]):[\\/]").expect("Failed to compile regex"));

pub fn path_windows_drive_letter_to_upper(abs_file_path: &str) -> String {
    RE_WINDOWS_PATH_DRIVE_LETTER
        .replace(abs_file_path, |caps: &regex::Captures| {
            format!("{}:", &caps[1].to_uppercase())
        })
        .to_string()
}
