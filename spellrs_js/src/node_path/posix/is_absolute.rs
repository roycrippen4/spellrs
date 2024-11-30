pub(crate) fn is_absolute(path: &str) -> bool {
    !path.is_empty() && path.starts_with('/')
}
