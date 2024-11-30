use url::Url;

use crate::{Posix, Windows};

use crate::node_path::Res;

pub(crate) fn path_to_file_url(filepath: &str, windows: Option<bool>) -> Res<Url> {
    let is_windows = windows.unwrap_or(cfg!(target_os = "windows"));

    if is_windows && filepath.starts_with(r"\\") {
        // UNC path format: \\server\share\resource
        let is_extended_unc = filepath.starts_with(r"\\?\UNC\");
        let prefix_length = if is_extended_unc { 8 } else { 2 };

        let hostname_end_index = filepath[prefix_length..]
            .find('\\')
            .map(|i| i + prefix_length)
            .ok_or_else(|| format!("Invalid path: '{}' - Missing UNC resource path", filepath))?;

        if hostname_end_index == 2 {
            return Err(format!("Invalid path: '{}' - Empty UNC server name", filepath).into());
        }

        let hostname = &filepath[prefix_length..hostname_end_index];
        let resource_path = &filepath[hostname_end_index..];
        let url_str = format!("file://{}/{}", hostname, resource_path);
        return Url::parse(&url_str)
            .map_err(|e| format!("Failed to parse URL from path '{}': {}", filepath, e).into());
    }

    // Resolve the path (normalize it to an absolute path)
    let mut resolved = match windows {
        Some(_) => Windows.resolve(&[filepath]),
        None => Posix.resolve(&[filepath]),
    };

    let file_path_last = filepath.chars().last();
    if let Some(last_char) = file_path_last {
        if (last_char == '/' || (is_windows && last_char == '\\'))
            && !resolved.ends_with(std::path::MAIN_SEPARATOR)
        {
            resolved.push('/');
        }
    }

    // Construct the file URL
    Url::from_file_path(&resolved).map_err(|_| {
        format!(
            "Failed to construct file URL from resolved path '{}'",
            resolved
        )
        .into()
    })
}
