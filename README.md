[![mirror](https://img.shields.io/badge/mirror-github-blue)](https://github.com/neilg63/substring-replace)
[![crates.io](https://img.shields.io/crates/v/substring-replace.svg)](https://crates.io/crates/substring-replace)
[![docs.rs](https://docs.rs/substring-replace/badge.svg)](https://docs.rs/substring-replace)

# substring-replace: Extract, insert and replace substrings

This crate adds a set of convenient methods to easily extract, insert and replace string slices in Rust with character indices compatibile with multibyte characters.

Do not add this library to your project if it already depends on the [substring](https://crates.io/crates/substring) crate. Its core substring method, while sharing the same signature and functionality, will conflict with the same method in the ```SubstringReplace``` trait, although the implementation in this crate avoids an unsafe block and will not panic if the start and end indices are out of range.

Regular Rust prefers ```str``` slices for extracting string by index ranges. However, it will panic when indices are out of range and works with byte indices rather than the more intuitive character indices as used with the [Regex](https://crates.io/crates/regex) crate. 

### substring

Returns a substring by start and end character index. With multibyte characters this will not be the same as the byte indices.

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
This will return the end of a string (```str``` or ```string```) from the specified start character index.
```rust
let sample_str = "/long/file/path";
let result = sample_str.substring_start(5);
// the result is "/file/path"
```


### substring_replace

```rust
let new_string = "azdefgh".substring_replace("bc", 1, 2);
println!("{}", new_string);
// will print "abcdefgh"
```

#### substring_replace_end
This replacse the remainder of string from a specified start character index
```rust
let new_string = "abcdefgh".substring_replace_end("xyz", 3);
println!("{}", new_string);
// will print "abcxyz"
```

### substring_offset
This extract a substring from a start index for n characters to the right or left.
A negative length in the second parameter will start at the start index
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
This convert characters index to a start byte index and is mainly used internally

```rust
let byte_index = "नमस्ते".to_start_byte_index(2);
// yields byte index of the third multibyte character. It should be 6
```

### to_end_byte_index
This convert characters index to an end byte index and is mainly used internally

### char_len
This returns the character length in terms of indivual unicode symbols as opposed to byte length with ```str::len()```.

```rust
let emoji = "😎";

println("Emoji length: {:?}, emoji byte length: {:?}", emoji.char_len(), emoihi.len() );
// prints: Emoji length: 1, emoji byte length: 4
```

### NB: This is alpha release, but the crate is feature complete and supplements [string-patterns](https://crates.io/crates/string-patterns) and [simple-string-patterns](https://crates.io/crates/simple-string-patterns) .