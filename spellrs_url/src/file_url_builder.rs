use crate::{
    add_trailing_slash, is_url_like, normalize_windows_url, path_windows_drive_letter_to_upper,
    to_filepath_or_href, url_parent, url_to_url_relative, IsUrlLike, StUrl, RE_WINDOWS_PATH,
};

use spellrs_js::{globals::decode_uri_component, NodePath, JS};
use url::Url;

#[derive(Debug)]
pub struct FileUrlBuilder {
    windows: bool,
    path: NodePath,
    cwd: Url,
}

impl Default for FileUrlBuilder {
    fn default() -> Self {
        Self::new(Some(cfg!(windows)), None)
    }
}

impl FileUrlBuilder {
    pub fn new(windows: Option<bool>, cwd: Option<Url>) -> Self {
        let windows = windows.unwrap_or(cfg!(windows));
        let path = NodePath::default();

        let cwd = match cwd {
            Some(cwd) => cwd,
            None => {
                let resolved = path.resolve(&[]) + "/";
                let root = _root_file_url(None, &path, windows);
                _path_to_file_url(&resolved, Some(&root.into()), windows, None)
            }
        };

        Self { windows, path, cwd }
    }

    pub fn encode_path_chars(&self, filepath: &str) -> String {
        encode_path_chars(filepath, self.windows)
    }

    pub fn normalize_filepath_for_url(&self, filepath: &str) -> String {
        _normalize_filepath_for_url(filepath, self.windows)
    }

    pub fn to_file_url(&self, filename_or_url: &StUrl, relative_to: Option<&StUrl>) -> Url {
        let url = _to_file_url(
            filename_or_url,
            relative_to,
            &self.cwd,
            self.windows,
            &self.path,
        );
        normalize_windows_url(&url.into()).expect("to be a valid url")
    }

    pub fn to_fil_dir_url(&self, dir_or_url: &StUrl, relative_to: Option<&StUrl>) -> Url {
        let url = self.to_file_url(dir_or_url, relative_to);
        add_trailing_slash(url)
    }

    pub fn url_to_filepath_or_href(&self, url: &StUrl) -> String {
        let url = self.to_file_url(url, None);
        _url_to_filepath_or_href(&url)
    }

    pub fn relative(&self, url_from: &Url, url_to: &Url) -> String {
        if url_from.scheme() == url_to.scheme() && url_from.scheme() == "file" {
            if url_from.as_str() == url_to.as_str() {
                return "".to_string();
            }

            let url_from = match url_from.as_str().ends_with('/') {
                true => url_from,
                false => &url_from.join("./").expect("to be a valid url"),
            };
            // let from_path = url_from.as_str();
            // let to_path = url_to.as_str();
            let from_path = url_from.path();
            let to_path = url_to.path();

            if to_path.starts_with(from_path) {
                let slice = to_path.slice(from_path.len() as isize, to_path.len() as isize);
                return decode_uri_component(slice);
            }

            let p_from = _url_to_filepath_or_href(url_from);
            let p_to = _url_to_filepath_or_href(url_to);
            let to_is_dir = url_to.as_str().ends_with('/');
            let mut pathname = self.normalize_filepath_for_url(&self.path.relative(&p_from, &p_to));
            if to_is_dir && !pathname.ends_with('/') {
                pathname += "/";
            }
            return decode_uri_component(&pathname);
        }

        decode_uri_component(&url_to_url_relative(url_from, url_to))
    }

    pub fn url_dirname(&self, url: &StUrl) -> Url {
        let url = StUrl::Url(self.to_file_url(url, None));
        url_parent(&url).expect("to be a valid url")
    }

    pub fn path_to_file_url(&self, pathname: &str, relative_to: Option<&StUrl>) -> Url {
        let normalized = self.normalize_filepath_for_url(pathname);
        let url = match relative_to {
            Some(url) => url.as_url().expect("to be a valid url"),
            None => self.cwd.clone(),
        };
        url.join(&normalized).expect("to be a valid url")
    }

    pub fn root_file_url(&self, filepath: Option<&str>) -> Url {
        let filepath = filepath.unwrap_or(".");
        let resolved = self.path.resolve(&[filepath]);
        let normalized = self.path.normalize(&resolved);
        let p = self.path.parse(&normalized);
        let normalized_for_url = self.normalize_filepath_for_url(&p.root);
        Url::parse(_get_fs_root_url(&self.path).as_str())
            .expect("to be a valid url")
            .join(&normalized_for_url)
            .expect("to be a valid url")
    }

    pub fn is_absolute(&self, filepath: &str) -> bool {
        is_url_like(filepath) || self.path.is_absolute(filepath)
    }

    pub fn is_url_like(url: impl IsUrlLike) -> bool {
        is_url_like(url)
    }
}

fn _url_to_filepath_or_href(url: &Url) -> String {
    if url.scheme() != "file" {
        return url.to_string();
    }

    let url = StUrl::Url(url.clone());
    let p = to_filepath_or_href(&url);
    // let p = decode_uri_component(&url.path().replace('/', "\\"));
    path_windows_drive_letter_to_upper(RE_WINDOWS_PATH.replace(&p, "$1").as_ref())
}

fn encode_path_chars(filepath: &str, windows: bool) -> String {
    filepath
        .chars()
        .flat_map(|ch| match ch {
            '%' => "%25".chars().collect::<Vec<_>>(),
            '\\' if !cfg!(windows) && !windows => "%5C".chars().collect::<Vec<_>>(),
            '\n' => "%0A".chars().collect::<Vec<_>>(),
            '\r' => "%0D".chars().collect::<Vec<_>>(),
            '\t' => "%09".chars().collect::<Vec<_>>(),
            other => vec![other],
        })
        .collect()
}

fn _normalize_filepath_for_url(filepath: &str, windows: bool) -> String {
    let mut f = encode_path_chars(filepath, windows)
        .replace('?', "%3F")
        .replace('#', "%23")
        .replace('\\', "/");

    if let Some((drive, rest)) = f.split_once(':') {
        if drive.len() == 1 && drive.chars().all(|c| c.is_ascii_alphabetic()) {
            f = format!("/{}/{}", drive.to_uppercase(), rest);
        }
    }

    f
}

fn _path_to_file_url(
    pathname: &str,
    relative_to: Option<&StUrl>,
    windows: bool,
    cwd: Option<&Url>,
) -> Url {
    let normalized = _normalize_filepath_for_url(pathname, windows);
    let url = match relative_to {
        Some(url) => url.as_url().expect("to be a valid url"),
        None => match cwd {
            Some(cwd) => cwd.clone(),
            None => unreachable!(),
        },
    };
    url.join(&normalized).expect("to be a valid url")
}

fn _root_file_url(filepath: Option<&str>, path: &NodePath, windows: bool) -> Url {
    let filepath = filepath.unwrap_or(".");
    let resolved = path.resolve(&[filepath]);
    let normalized = path.normalize(&resolved);
    let p = path.parse(&normalized);
    let normalized_for_url = _normalize_filepath_for_url(&p.root, windows);
    Url::parse(path.to_file_url("/").expect("to be a url").as_str())
        .expect("to be a valid url")
        .join(&normalized_for_url)
        .expect("to be a valid url")
}

fn _to_file_url(
    filename_or_url: &StUrl,
    relative_to: Option<&StUrl>,
    cwd: &Url,
    windows: bool,
    path: &NodePath,
) -> Url {
    if let StUrl::Url(url) = filename_or_url {
        return url.clone();
    };

    if is_url_like(filename_or_url.as_str()) {
        return filename_or_url.as_url().expect("to be a valid url");
    }

    let relative_to = match relative_to {
        Some(relative_to) => relative_to,
        None => &StUrl::Url(cwd.clone()),
    };

    let filename_or_url = match cfg!(windows) {
        true => filename_or_url.as_str().replace('\\', "/"),
        false => filename_or_url.to_string(),
    };

    if is_url_like(relative_to.clone()) {
        let pathname = _normalize_filepath_for_url(&filename_or_url, windows);
        let url = relative_to.as_url().expect("to be a valid url");
        return url.join(&pathname).expect("to be a valid url");
    }

    let append_slash = match filename_or_url.ends_with('/') {
        true => "/",
        false => "",
    };

    let resolved = path.resolve(&[relative_to.as_str(), &filename_or_url]);
    let pathname = _normalize_filepath_for_url(&resolved, windows) + append_slash;
    _path_to_file_url(&pathname, Some(&StUrl::Url(cwd.clone())), windows, None)
}

fn _get_fs_root_url(path: &NodePath) -> Url {
    path.to_file_url("/").expect("to be a url")
}

#[cfg(test)]
#[allow(unused)]
mod test {
    use crate::StUrl;

    use super::FileUrlBuilder;
    use once_cell::sync::Lazy;
    use spellrs_js::NodePath;
    use url::Url;

    static PATH: Lazy<NodePath> = Lazy::new(NodePath::default);

    #[test]
    fn test_builder() {
        let builder = FileUrlBuilder::new(None, None);
        let expected = builder
            .path
            .path_to_file_url("./", None)
            .expect("to be a url")
            .as_str()
            .to_lowercase()
            + PATH.sep();
        let result = builder.cwd.as_str().to_lowercase();
        assert_eq!(expected, result);

        let url = StUrl::Url(builder.cwd.clone());
        let result = builder.url_to_filepath_or_href(&url).to_lowercase();
        let expected = PATH.resolve(&["."]).to_lowercase() + PATH.sep();
        assert_eq!(expected, result);
    }

    // #[test]
    // fn test_builder_relative() {
    //     #[rustfmt::skip]
    //     let cases = [
    //         (".", ".", ""),
    //         ("e:/path/to/file.txt", "e:/path/to/file2.txt", "file2.txt"),
    //         ("file:///E:/user/test/project/deeper/", "file:///E:/user/Test/project/", "../"),
    //         ("file:///E:/user/Test/project/", "file:///E:/user/Test/project//deeper/", "deeper/"),
    //     ];

    //     for (i, (from, to, expected)) in cases.iter().enumerate() {
    //         let builder = FileUrlBuilder::default();
    //         let from = builder.path_to_file_url(from, None);
    //         let to = builder.path_to_file_url(to, None);
    //         let result = builder.relative(&from, &to);

    //         assert_eq!(
    //             &result, expected,
    //             "\n\nCASE {i} FAILED\nFROM    : \"{from}\"\nTO      : \"{to}\"\nresult  : {:?}\nexpected: {:?}\n\n",
    //             result, expected
    //         );
    //     }
    // }
}
