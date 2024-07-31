[![mirror](https://img.shields.io/badge/mirror-github-blue)](https://github.com/neilg63/substring-replace)
[![crates.io](https://img.shields.io/crates/v/substring-replace.svg)](https://crates.io/crates/substring-replace)
[![docs.rs](https://docs.rs/substring-replace/badge.svg)](https://docs.rs/substring-replace)

# substring-replace: Extract, insert and replace substrings

This crate adds a set of convenient methods to easily extract, insert and replace string slices in Rust with character indices compatibile with multibyte characters.

Do not add this library to your project if it already depends on the [substring](https://crates.io/crates/substring) crate. Its core substring method will conflict with the same method in the ```SubstringReplace``` trait. The like-named modules share the same signature and functionality, although this crate avoids unsafes block and will not panic if the start and end indices are out of range.

Regular Rust prefers ```str``` slices for extracting string by index ranges. However, it will panic when indices are out of range and works with byte indices rather than the more intuitive character indices as used with the [Regex](https://crates.io/crates/regex) crate. 

### substring

Returns a substring between start and end character indices. These indices differ from byte indices with multibyte characters in the extended Latin-script, most non-Latin alphabets, many special symbols and emojis.

```rust
let sample_str = "/long/file/path";
let result = sample_str.substring(5,9);
// the result is "file"
```

### substring_start

This will return the start of a string (```str``` or ```string```) until the specified end character index.
```rust
let sample_str = "/long/file/path";
let result = sample_str.substring_start(5);
// the result is "/long"
```


### substring_end

This method returns the end of a string (```str``` or ```string```) from the specified start character index.
```rust
let sample_str = "/long/file/path";
let result = sample_str.substring_start(5);
// the result is "/file/path"
```


### substring_replace

This method removes characters between the specified start and end indices and inserts a replacement string
```rust
let new_string = "azdefgh".substring_replace("bc", 1, 2);
println!("{}", new_string);
// will print "abcdefgh"
```

#### substring_replace_start

This method replaces the start of a string to a specified end character index
```rust
// remove the first 2 characters and prepend the string "xyz"
let new_string = "abcdefgh".substring_replace_start("xyz", 2);
println!("{}", new_string);
// will print "xyzcdefgh"
```

#### substring_replace_end

This method replaces the remainder of string from a specified start character index
```rust
// remove all characters after and index of 3 and append the string "xyz"
let new_string = "abcdefgh".substring_replace_end("xyz", 3);
println!("{}", new_string);
// will print "abcxyz"
```

### substring_remove
This method returns the remainder after removing a substring delimited by start and end character indices.
```rust
let sample_str = "abcdefghij";
let result = sample_str.substring_remove(3, 6);
// result will be "abcfghij"
```

### substring_offset
This method extracts a substring from a start index for n characters to the right or left.
A negative length in the second parameter will start at the start index
```rust
let sample_str = "indian-elephant";
let result = sample_str.substring_offset(7, 3);
// result will be "ele"
```

### substring_pull
This method returns the remainder after removing a substring from a start index for n characters to the right or left.
As with siubstring_offset, a negative length in the second parameter will start at the start index
```rust
let sample_str = "indian-elephant";
let result = sample_str.substring_offset(7, 3);
// result will be "ele"
```

### substring_insert

This method inserts a string at a given character index and differs from the standard ```String::insert``` method by using character rather than byte indices to work better with multibyte characters. It also works directly with ```&str```, but returns a new owned string.

```rust
let sample_str = "a/c";
let result = sample_str.substring_insert("/b", 1);
// result will be "a/b/c"
```

### to_start_byte_index

This convert a start character index to a start byte index. It's mainly used internally.
It differs only from the ```to_end_byte_index``` in its default value of 0 if it overflows.
```rust
let byte_index = "नमस्ते".to_start_byte_index(2);
// yields byte index of at the start of third multibyte character (character index 2). It should be 6
```

### to_end_byte_index

This method converts an end character index to an end byte index. It's mainly used internally.
It differs only from the ```to_end_byte_index``` in its default value at the end if it overflows.

### char_len

This returns the character length in terms of individual unicode symbols as opposed to byte length with ```str::len()```.
This is shorthand for ```&str::char_indices().count()```.
```rust
let emoji = "😎";
println!("Emoji length: {}, emoji byte length: {}", emoji.char_len(), emoji.len() );
// prints: Emoji length: 1, emoji byte length: 4
```

---

NB: This is an alpha release, but the crate is feature-complete and supplements [string-patterns](https://crates.io/crates/string-patterns) and [simple-string-patterns](https://crates.io/crates/simple-string-patterns) .

### Version history

**1.3:** Added new methods ```.substring_remove(start: usize, end: usize)``` and ```.substring_pull(position: usize, length: i32)```.