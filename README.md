[![mirror](https://img.shields.io/badge/mirror-github-blue)](https://github.com/neilg63/substring-replace)
[![crates.io](https://img.shields.io/crates/v/substring-replace.svg)](https://crates.io/crates/substring-replace)
[![docs.rs](https://docs.rs/substring-replace/badge.svg)](https://docs.rs/substring-replace)

Here's an enhanced version of your README for better readability and structure:

# substring-replace: Substring Manipulation for Rust

This crate adds intuitive substring methods for manipulating Rust string slices (`&str`) with character indices, handling multibyte characters with ease, similar to `substring()` in JavaScript, Java, or C#, and `substr()` in C++ or PHP.

## Features

- **Core `substring` Method**: Compatible with the simpler [substring](https://crates.io/crates/substring) crate but includes additional safe handling for out-of-range indices and negative offsets.

- **Avoiding Conflicts**: Do not use this crate alongside the existing `substring` crate. If you need advanced features, replace `use substring::*;` with `use substring_replace::*;` after removing the `substring` dependency.

- **Character vs Byte Indices**: Unlike Rust's standard string slicing by byte indices, this crate uses character indices for intuitive handling, especially with Unicode.

## Key Methods

### `substring(start: usize, end: i64) -> &str`

Extracts a substring based on character indices. Negative end indices count from the string's end.

```rust
let sample_str = "/long/file/path";
assert_eq!(sample_str.substring(5, 9), "file");
```

### `substring_replace(replacement: &str, start: usize, end: i64) -> String`

Replaces a substring with another, using character indices.

```rust
let new_str = "azdefgh".substring_replace("bc", 1, 2);
assert_eq!(new_str, "abcdefgh");
```

### `substring_insert(insertion: &str, index: usize) -> String`

Inserts a string at a character index.

```rust
let result = "a/c".substring_insert("/b", 1);
assert_eq!(result, "a/b/c");
```

### `substring_start(end: i64) -> &str` and `substring_end(start: i64) -> &str`

Extract substrings from the start or end based on character indices.

```rust
let sample_str = "/long/file/path";
assert_eq!(sample_str.substring_start(5), "/long");
assert_eq!(sample_str.substring_end(5), "/file/path");
```

### `substring_replace_start(replacement: &str, end: i64) -> String` and `substring_replace_end(replacement: &str, start: i64) -> String`

Replaces from the start or end up to a specified index.

```rust
assert_eq!("abcdefgh".substring_replace_start("xyz", 2), "xyzcdefgh");
```

### `substring_remove(start: usize, end: usize) -> String`

Removes a substring by indices.

```rust
assert_eq!("abcdefghij".substring_remove(3, 6), "abcfghij");
```

### `substring_offset(position: usize, length: i32) -> &str` and `substring_pull(position: usize, length: i32) -> String`

Extract or remove substrings based on a start position and length.

```rust
assert_eq!("indian-elephant".substring_offset(7, 3), "ele");
```

### `to_start_byte_index(index: usize) -> usize` and `to_end_byte_index(index: i64) -> usize`

Convert character indices to byte indices for slice creation.

```rust
let byte_index = "नमस्ते".to_start_byte_index(2);
```

### `char_len() -> usize`

Returns the character count of a string.

```rust
println!("{}", "😎".char_len()); // prints 1
```

### `char_find(pattern: &str) -> String`

This finds the first character index of a plain string pattern. Like the standard _find_ method, it returns an optional unsigned integer (usize). To search from right to left, but still returning the index of the first character in the matched sequence, you can use `char_rfind`,

```rust
let greek_words = "μήλα και πορτοκάλια";
let search_word = "και";
let character_index = greek_words.char_find(search_word);
let byte_index = greek_words.find(search_word);
println!("The word {search_word} starts at a character index of {character_index} and a byte index of {byte_index}");
// The word $search_word starts at a character index of 5 and a byte index of 9
```

### insert_before_first, insert_before_last, insert_after_first, insert_after_last and insert_between

These new methods combine the functionality of char_find or char_rfind and substring_insert or substring_replace to insert strings before, after or between the first and/or last occurrence of a given sequence. The first four methods are wrappers for `insert_adjacent`.

```rust
let file_name = "greek-holiday-snap.jpg";
let new_file_name = file_name.insert_before_last("--cropped", ".");
// should be greek-holiday-snap--cropped.jpg
```

---

NB: This is an alpha release, but the crate is feature-complete and supplements [string-patterns](https://crates.io/crates/string-patterns) and [simple-string-patterns](https://crates.io/crates/simple-string-patterns) .

### Version history

**0.1.3:** Added new methods `.substring_remove(start: usize, end: usize)` and `.substring_pull(position: usize, length: i32)`.

**0.1.5:** Added new methods `.char_find(pat: &str)` and `.char_rfind(pat: &str)`.

**0.2.0:** The last parameter of `.substring_start(end: i64)`, `.substring_end(start: i64)`, `.substring_replace_start(replacement: &str, end: i64)` and `.substring_replace_end(replacement: &str, start: i64)` is now a 64-bit integer to let you assign a negative index as character offset from the end, e.g. "abcdefghi".substring_end(-3) would yield "ghi". Two new variant methods that accept negative end offsets were also introduced: `substring_range(start; usize, end: i64)` and `substring_replace_range(replacement: &str, start; usize, end: i64)`.

**0.2.1** Two new features:

- Added a set of convenience methods to insert _strings_ before or after the first or last character.
- The crate now depends on the ToOffset trait to allow the core `substring` and `substring_replace` methods to alow negative offsets in the last parameter (i32, i64) as well as the default usize. A a result, `substring_range` and `substring_replace_range` are deprecated.
