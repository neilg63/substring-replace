[![mirror](https://img.shields.io/badge/mirror-github-blue)](https://github.com/neilg63/substring-replace)
[![crates.io](https://img.shields.io/crates/v/substring-replace.svg)](https://crates.io/crates/substring-replace)
[![docs.rs](https://docs.rs/substring-replace/badge.svg)](https://docs.rs/substring-replace)

# substring-replace: Extract, insert and replace substrings

This crate adds a set of convenient methods to easily extract, insert and replace string slices in Rust with character indices compatibile with multibyte characters.

Do not add this library to your project if it already depends on the [substring](https://crates.io/crates/substring) crate. Its core substring method, while sharing the same signature and functionality, will conflict with the same method in the ```SubstringReplace``` trait, although the implementation in this crate avoids an unsafe block and will not panic if the start and end indices are out of range.

Regular Rust prefers ```str``` slices for extracting string by index ranges. However, it will panic when indices are out of range and works with byte indices rather than the more intuitive character indices as used with the [Regex](https://crates.io/crates/regex) crate. 

### substring

Returns a substring by start and end character index. With multibyte characters this will not be the same as the byte indices.

### ```substring_start

```rust
let sample_str = "/long/file/path";
let result = sample_str.substring_start(5,9);
// the result is "file"
```

### ```substring_end```


### ```substring_replace```
fn substring_replace_start(&self, replacement: &str, end: usize) -> String {
    self.substring_replace(replacement, 0, end)
}

    /// Replace the remainder of string from a specified start character index
    /// e.g. "blue".substring_replace_last("ack", 2);
    /// will replace the last 2 characters with "ack", yielding "black"
    fn substring_replace_end(&self, replacement: &str, start: usize) -> String {
        let end = self.char_len();
        self.substring_replace(replacement, start, end)
    }

    /// Extract a substring from a start index for n characters to the right
    /// A negative length in the second parameter will start at the start index
    fn substring_offset(&self, position: usize, length: i32) -> &str {
        let reverse = length < 0; 
        let start = if reverse {
            position.checked_sub(length.abs() as usize).unwrap_or(0)
        } else {
            position
        };
        let start_i32 =  if start > i32::MAX as usize { i32::MAX } else { start as i32 };
        let end_i32 = start_i32 + length.abs();
        let end = if end_i32 < 0 {
            0
        } else {
            end_i32 as usize
        };
        self.substring(start, end)
    }

    /// Insert a string at a given character index
    /// This differs from String::insert as it uses character rather than byte indices
    /// and thus works better with multibyte characters
    /// It's also implemented to str, while returning a new owned string
    fn substring_insert(&self, replacement: &str, start: usize) -> String {
        self.substring_replace(replacement, start, start)
    }

    /// Convert character index to start byte index
    fn to_start_byte_index(&self, start: usize) -> usize;

    /// Convert character index to end byte index
    fn to_end_byte_index(&self, start: usize) -> usize;

    /// Return the character length rather than the byte length
    fn char_len(&self) -> usize;