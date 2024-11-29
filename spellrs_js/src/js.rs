/// The `JS` trait provides JavaScript-inspired string manipulation methods
/// for `&str` and `String` in Rust. These methods mimic common operations
/// available on JavaScript's `String` type, including slicing, searching,
/// and retrieving character codes.
///
/// The trait is implemented for `&str` and can be used to extend the
/// functionality of `String` when working with Rust strings.
///
/// # Examples
/// ```rust
/// use my_crate::JS;
///
/// let s = "hello world";
///
/// // Get the Unicode code point of a character
/// assert_eq!(s.char_code_at(1), 101); // Unicode for 'e'
///
/// // Slice the string
/// assert_eq!(s.js_slice(0, 5), "hello");
///
/// // Find the index of a character
/// assert_eq!(s.index_of('o', None), Some(4));
/// ```
pub trait JS {
    /// Returns the Unicode code point of the character at the specified index `i`.
    /// If the index is out of bounds, returns `-1`.
    ///
    /// # Parameters
    /// - `i`: The zero-based index of the character whose Unicode code point is to be retrieved.
    ///
    /// # Returns
    /// - `i32`: The Unicode code point of the character, or `-1` if `i` is out of range.
    ///
    /// # Examples
    /// ```rust
    /// let s = "hello";
    /// assert_eq!(s.char_code_at(1), 101); // Unicode for 'e'
    /// assert_eq!(s.char_code_at(10), -1); // Out of bounds
    /// ```
    fn char_code_at(&self, i: usize) -> i32;

    /// Extracts a section of the string and returns it as a new slice.
    /// Supports negative indices for slicing from the end of the string.
    ///
    /// # Parameters
    /// - `start`: The starting index of the slice (inclusive). Negative values count back from the end of the string.
    /// - `end`: The ending index of the slice (exclusive). Negative values count back from the end of the string.
    ///
    /// # Returns
    /// - `Self`: A string slice containing the extracted section.
    ///
    /// # Examples
    /// ```rust
    /// let s = "hello world";
    /// assert_eq!(s.js_slice(0, 5), "hello"); // Extracts "hello"
    /// assert_eq!(s.js_slice(-5, -1), "worl"); // Extracts "worl" (negative indexing)
    /// ```
    fn slice(&self, start: isize, end: isize) -> Self;

    /// Returns the index of the first occurrence of a specified character `pat` in the string,
    /// starting the search at a specified position. Returns `None` if the character is not found.
    ///
    /// # Parameters
    /// - `pat`: The character to search for.
    /// - `position`: Optional starting index for the search. If `None`, the search begins at the start of the string.
    ///
    /// # Returns
    /// - `Option<usize>`: The zero-based index of the first occurrence of `pat`, or `None` if `pat` is not found.
    ///
    /// # Examples
    /// ```rust
    /// let s = "hello world";
    /// assert_eq!(s.index_of('o', None), Some(4)); // Finds 'o' at index 4
    /// assert_eq!(s.index_of('o', Some(5)), Some(7)); // Starts search at index 5, finds 'o' at index 7
    /// assert_eq!(s.index_of('x', None), None); // 'x' not found
    /// ```
    fn index_of(&self, pat: char, position: Option<usize>) -> Option<usize>;
}

impl JS for String {
    fn index_of(&self, pat: char, position: Option<usize>) -> Option<usize> {
        self.chars()
            .enumerate()
            .position(|(i, c)| c == pat && i >= position.unwrap_or(0))
    }

    fn char_code_at(&self, i: usize) -> i32 {
        self.chars().nth(i).map(|c| c as i32).unwrap_or(-1)
    }

    fn slice(&self, start: isize, end: isize) -> String {
        let len = self.chars().count() as isize;
        let start = if start < 0 { len + start } else { start };
        let end = if end < 0 { len + end } else { end };

        self.chars()
            .skip(start as usize)
            .take((end - start) as usize)
            .collect()
    }
}

impl JS for &str {
    fn slice(&self, start: isize, end: isize) -> Self {
        let len = self.chars().count() as isize;
        let start = if start < 0 { len + start } else { start };
        let end = if end < 0 { len + end } else { end };

        // Convert character indices to byte indices
        let start_idx = self
            .char_indices()
            .nth(start as usize)
            .map(|(idx, _)| idx)
            .unwrap_or(0);
        let end_idx = self
            .char_indices()
            .nth(end as usize)
            .map(|(idx, _)| idx)
            .unwrap_or(self.len());

        // Return the slice
        &self[start_idx..end_idx]
    }

    fn char_code_at(&self, i: usize) -> i32 {
        self.chars().nth(i).map(|c| c as i32).unwrap_or(-1)
    }

    fn index_of(&self, pat: char, position: Option<usize>) -> Option<usize> {
        self.chars()
            .enumerate()
            .position(|(i, c)| c == pat && i >= position.unwrap_or(0))
    }
}

#[test]
fn test_js_slice() {
    let path = "i am a string";
    let result = path.slice(-3, -1);
    assert_eq!("in".to_string(), result);
}
