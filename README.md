[![mirror](https://img.shields.io/badge/mirror-github-blue)](https://github.com/neilg63/substring-replace)
[![crates.io](https://img.shields.io/crates/v/substring-replace.svg)](https://crates.io/crates/substring-replace)
[![docs.rs](https://docs.rs/substring-replace/badge.svg)](https://docs.rs/substring-replace)

# substring-replace: Extract, insert and replace substrings

This crate adds developer-friendly substring methods to easily manipulate string slices in Rust with character indices compatibile with multibyte characters in a similar way to _substring()_ methods in Javascript, Java or C# or _substr()_ in C++ and PHP.

This crate's core **substring** method has the same signature and functionality as the simpler [substring](https://crates.io/crates/substring) crate, but adds many supplementary methods, such as `substring_replace`, avoids the need for unsafe blocks and fails gracefully if the start or end index is out of range. However, the two crates should not be added to the same project. If you only need the core **substring** method and already use the other well-supported crate, do not install this crate. On the other hand, if you need some of extra features available from this crate, uninstall the other crate before installing this one and replace `use substring::*;` with `use substring_replace::*;`.

Regular Rust prefers _slices_ to manipulate strings by byte index ranges. However, it panics when byte indices are out of range or fall between character boundaries. Character indices are more intuitive and compatible with the popular [Regex](https://crates.io/crates/regex) crate.

### substring

Returns a substring between start and end character indices. These indices differ from byte indices with multibyte characters in the extended Latin-script, most non-Latin alphabets, many special symbols and emojis.
A negative offset in the last parameter counts backwards from the end.

```rust
let sample_str = "/long/file/path";
let result = sample_str.substring(5,9);
// the result is "file"

let sample_str = "2024-10-12T17:32:43.00Z";
let result = sample_str.substring(4,-4);
// the result is "10-12T17:32:43"
```

### substring_replace

This method removes characters between the specified start and end indices and inserts a replacement string. A negative offset in the last parameter counts backwards from the end.

```rust
let new_string = "azdefgh".substring_replace("bc", 1, 2);
println!("{}", new_string);
// will print "abcdefgh"

let new_string = "abcdefgh".substring_replace_range("_xyz_", 2, -2);
// replace all characters after the first two and before the last 2
println!("{}", new_string);
// will print "ab_xyz_gh"

```

### substring_insert

This method inserts a string at a given character index and differs from the standard `String::insert` method by using character rather than byte indices to work better with multibyte characters. It also works directly with `&str`, but returns a new owned string.

```rust
let sample_str = "a/c";
let result = sample_str.substring_insert("/b", 1);
// result will be "a/b/c"
```

### substring_start and substring_end

_substring_start_ returns the start of a string (`str` or `string`) until the specified end character index,
while _substring_end_ returns the end of a string (`&str` or `string`) from the specified start character index.
Like the standard _substring_ method, "substring*range" accepts a start and end range, but the end index may be negative.
A negative offset represents character index from the end, e.g. if the character length is 15, -5 translates to 10.
This is useful with \_substring_end* when you know how many characters you need to capture, but do not want to check the string character length first and with _substring_start_ when you know how many characters you need to remove from the end.

```rust
let sample_str = "/long/file/path";
let result_1 = sample_str.substring_start(5);
// the result is "/long"

let result_2 = sample_str.substring_end(5);
// the result is "/file/path"

let result_3 = sample_str.substring_end(-4);
// the result is "path"
```

### substring_replace_start and substring_replace_end

_substring_replace_start_ replaces the start of a string to a specified end character index, while _substring_replace_end_ replaces the remainder of string from a specified start character index.
In both methods, a negative index value in the last parameter means that many characters before the end of the string, e.g. if the character length is 15, -5 translates to 10

```rust
// remove the first 2 characters and prepend the string "xyz"
let new_string = "abcdefgh".substring_replace_start("xyz", 2);
println!("{}", new_string);
// will print "xyzcdefgh"

let new_string = "abcdefgh".substring_replace_end("xyz", 3);
// remove all characters after and index of 3 and append the string "xyz"
println!("{}", new_string);
// will print "abcxyz"
```

### substring_remove

This method returns the remainder after removing a substring delimited by start and end character indices.
It's the oposite to **substring(start, end)**.

```rust
let sample_str = "abcdefghij";
let result = sample_str.substring_remove(3, 6);
// result will be "abcfghij"
```

### substring_offset

This method extracts a substring from a start index for n characters to the right or left.
A negative length in the second parameter will end at the reference index.

```rust
let sample_str = "indian-elephant";
let result = sample_str.substring_offset(7, 3);
// result will be "ele"
```

### substring_pull

This method returns the remainder after removing a substring from a start index for _n_ characters to the right or left.
It's the oposite to **substring_offset(position, length)**.
As with **substring_offset**, a negative length in the second parameter will end at the reference index.

```rust
let sample_str = "three-horse-race";
let result = sample_str.substring_pull(5, 6);
// result will be "three-race"
```

### to_start_byte_index and to_end_byte_index

Theses methods convert either a start character index into a start byte index or an end character index into an end byte index. They're mainly used internally to build a string slice.
They differ only in their default value. For `to_start_byte_index` the default value is 0, while for `to_end_byte_index` it's the endmost index.

```rust
let byte_index = "नमस्ते".to_start_byte_index(2);
// yields byte index of at the start of third multibyte character (character index 2). It should be 6
```

### char_len

This returns the character length in terms of individual unicode symbols as opposed to byte length with `str::len()`.
This is shorthand for `&str::char_indices().count()`.

```rust
let emoji = "😎";
println!("Emoji length: {}, emoji byte length: {}", emoji.char_len(), emoji.len() );
// prints: Emoji length: 1, emoji byte length: 4
```

### char_find

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
