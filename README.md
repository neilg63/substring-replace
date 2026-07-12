[![mirror](https://img.shields.io/badge/mirror-github-blue)](https://github.com/neilg63/substring-replace)
[![crates.io](https://img.shields.io/crates/v/substring-replace.svg)](https://crates.io/crates/substring-replace)
[![docs.rs](https://docs.rs/substring-replace/badge.svg)](https://docs.rs/substring-replace)

# substring-replace: Substring Manipulation for Rust

This crate adds intuitive substring methods for manipulating Rust strings with character indices, handling multibyte characters with ease, similar to `substring()` in JavaScript, Java, or C#, and `substr()` in C++ or PHP.

## Features

- **Core `substring` Method**: Compatible with the simpler [substring](https://crates.io/crates/substring) crate but includes additional safe handling for out-of-range indices and negative offsets.

- **Avoiding Conflicts**: Do not use this crate alongside the existing `substring` crate. If you need advanced features, replace `use substring::*;` with `use substring_replace::*;` after removing the `substring` dependency.

- **Character vs Byte Indices**: Unlike Rust's standard string slicing by byte indices, this crate uses character indices for intuitive handling, especially with Unicode.

- **Works with any string type**: The `SubstringReplace` trait is implemented generically for any `T: AsRef<str>`, so `&str`, `String`, `Cow<str>` and `Box<str>` all get the same methods.

- **No regular expressions**: Every method matches literal substrings only. That's deliberate — this crate is for predictable, fixed patterns (file names, delimiters, extensions), not general pattern matching. If you need character classes, alternation or quantifiers, reach for the `regex` crate instead.

## Key Methods

### `substring<T: ToOffset>(start: usize, end: T) -> &str`

Extracts a substring based on character indices. `end` accepts any integer type implementing `ToOffset` (`i32`, `i64`, `u8`, `u16`, `u32`, `u64`, `usize`); a negative value counts back from the end of the string.

```rust
let sample_str = "/long/file/path";
assert_eq!(sample_str.substring(6, 10), "file");
```

### `substring_replace<T: ToOffset>(replacement: &str, start: usize, end: T) -> String`

Replaces a substring with another, using character indices.

```rust
let new_str = "azdefgh".substring_replace("bc", 1, 2);
assert_eq!(new_str, "abcdefgh");
```

### `substring_insert(replacement: &str, start: usize) -> String`

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
assert_eq!("abcde".substring_remove(2, 4), "abe");
```

### `substring_offset(position: usize, length: i32) -> &str` and `substring_pull(position: usize, length: i32) -> String`

Extract or remove substrings based on a start position and length. A negative `length` extends to the left of `position` instead of to the right.

```rust
assert_eq!("indian-elephant".substring_offset(7, 3), "ele");
```

### `to_start_byte_index(index: usize) -> usize` and `to_end_byte_index(index: usize) -> usize`

Convert character indices to byte indices for slice creation. An index at or beyond the character length maps to the end of the string.

```rust
let byte_index = "नमस्ते".to_start_byte_index(2);
assert_eq!(byte_index, 6); // each Devanagari character in this word is 3 bytes
```

### `char_len() -> usize`

Returns the character count of a string.

```rust
println!("{}", "😎".char_len()); // prints 1
```

### `char_find(pattern: &str) -> Option<usize>` and `char_rfind(pattern: &str) -> Option<usize>`

Return the *character* index (not byte index) of the first or last occurrence of a literal pattern. Both return the index where the match *starts*. An empty pattern always yields `None` — there is nothing to match. Like the standard library's `find`/`rfind`, they return `None` if the pattern isn't present.

```rust
let greek_words = "μήλα και πορτοκάλια";
let search_word = "και";
if let Some(character_index) = greek_words.char_find(search_word) {
    let byte_index = greek_words.find(search_word).unwrap();
    println!("The word {search_word} starts at a character index of {character_index} and a byte index of {byte_index}");
    // The word και starts at a character index of 5 and a byte index of 9
}
```

### `insert_before_first`, `insert_before_last`, `insert_after_first`, `insert_after_last` and `insert_between`

These combine `char_find`/`char_rfind` with `substring_insert`/`substring_replace` to insert strings before, after, or between the first and/or last occurrence of a given literal pattern. If the pattern isn't found, the original string is returned unchanged. The first four are thin wrappers around the lower-level `insert_adjacent(insert: &str, pat: &str, before: bool, first: bool) -> String`.

```rust
fn insert_before_first(&self, insert: &str, pat: &str) -> String;
fn insert_before_last(&self, insert: &str, pat: &str) -> String;
fn insert_after_first(&self, insert: &str, pat: &str) -> String;
fn insert_after_last(&self, insert: &str, pat: &str) -> String;
fn insert_between(&self, insert: &str, start_pat: &str, end_pat: &str) -> String;
```

```rust
let file_name = "greek-holiday-snap.jpg";
let new_file_name = file_name.insert_before_last("--cropped", ".");
// greek-holiday-snap--cropped.jpg
```

### `prepend(insert: &str) -> String` and `append(insert: &str) -> String`

Small convenience wrappers that add a string to the very start or end.

```rust
assert_eq!("world".prepend("hello "), "hello world");
assert_eq!("hello".append(" world"), "hello world");
```

## Benchmarks

`cargo bench` runs a [Criterion](https://github.com/bheisler/criterion.rs) benchmark (dev-only — it doesn't affect the published dependency tree) comparing `insert_before_last` against a hand-written, `std`-only equivalent, in `benches/insert_before_last.rs`.

---

### Version history

**0.2.4** The trait is now implemented generically for any `T: AsRef<str>` (`&str`, `String`, `Cow<str>`, `Box<str>`, etc.) rather than `&str` alone. Fixed three bugs: `char_find`/`char_rfind` could return the wrong index — or a false match — for patterns with a repeated leading character; an empty search pattern caused a panic (now returns `None`); and inserting or replacing exactly at the end of a string silently discarded the preceding content.

**0.2.2** Switched to MIT licence.

**0.1.3:** Added new methods `.substring_remove(start: usize, end: usize)` and `.substring_pull(position: usize, length: i32)`.

**0.1.5:** Added new methods `.char_find(pat: &str)` and `.char_rfind(pat: &str)`.

**0.2.0:** The last parameter of `.substring_start(end: i64)`, `.substring_end(start: i64)`, `.substring_replace_start(replacement: &str, end: i64)` and `.substring_replace_end(replacement: &str, start: i64)` is now a 64-bit integer to let you assign a negative index as character offset from the end, e.g. "abcdefghi".substring_end(-3) would yield "ghi". Two new variant methods that accept negative end offsets were also introduced: `substring_range(start: usize, end: i64)` and `substring_replace_range(replacement: &str, start: usize, end: i64)`.

**0.2.1** Two new features:

- Added a set of convenience methods to insert _strings_ before or after the first or last character.
- The crate now depends on the ToOffset trait to allow the core `substring` and `substring_replace` methods to allow negative offsets in the last parameter (i32, i64) as well as the default usize. As a result, `substring_range` and `substring_replace_range` are deprecated.
