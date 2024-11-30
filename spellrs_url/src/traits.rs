use once_cell::sync::Lazy;
use regex::Regex;
use url::Url;

use crate::StUrl;

static SCHEME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z][a-zA-Z0-9+\-.]*:").unwrap());

/// A trait for determining if a value is URL-like.
///
/// This trait is implemented for `String`, `Url` and '&str', enabling the use of the
/// `is_url_like` function to check whether a value represents a URL or resembles one.
///
/// # Examples
/// ```
/// use url::Url;
/// use spellrs_url::traits::*;
///
/// let url = Url::parse("https://example.com").unwrap();
/// let string = "https://example.com".to_string();
///
/// assert!(url.is_url_like());
/// assert!(string.is_url_like());
/// ```
pub trait IsUrlLike {
    /// Checks whether the value is URL-like.
    ///
    /// - For `Url`, this always returns `true`.
    /// - For `String`, this returns `true` if the string resembles a URL.
    fn is_url_like(&self) -> bool;

    /// Checks whether the value is _NOT_ URL-like.
    ///
    /// This is the logical negation of `is_url_like`.
    fn is_not_url_like(&self) -> bool {
        !self.is_url_like()
    }
}

/// Implementation of `IsUrlLike` for `String`.
///
/// Checks if the string resembles a URL using a regex.
impl IsUrlLike for String {
    fn is_url_like(&self) -> bool {
        SCHEME_REGEX.is_match(self)
    }
}

/// Implementation of `IsUrlLike` for `&str`.
///
/// Checks if the string slice resembles a URL using a regex.
impl IsUrlLike for &str {
    fn is_url_like(&self) -> bool {
        SCHEME_REGEX.is_match(self)
    }
}

/// Implementation of `IsUrlLike` for `Url`.
///
/// Always returns `true` since `Url` is inherently URL-like.
impl IsUrlLike for Url {
    fn is_url_like(&self) -> bool {
        true // Always true for Url objects
    }
}

impl IsUrlLike for StUrl<'_> {
    fn is_url_like(&self) -> bool {
        SCHEME_REGEX.is_match(self.as_str())
    }
}
