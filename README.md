[![mirror](https://img.shields.io/badge/mirror-github-blue)](https://github.com/neilg63/substring-replace)
[![crates.io](https://img.shields.io/crates/v/substring-replace.svg)](https://crates.io/crates/substring-replace)
[![docs.rs](https://docs.rs/substring-replace/badge.svg)](https://docs.rs/substring-replace)

# substring-replace: Extract, insert and replace substrings

This crate adds developer-friendly substring methods to easily manipulate string slices in Rust with character indices compatibile with multibyte characters in a similar way to *substring()* methods in Javascript, Java or C# or *substr()* in C++ and PHP.

This crate's core **substring** method has the same signature and functionality as the simpler [substring](https://crates.io/crates/substring) crate, but adds many supplementary methods, such as ```substring_replace``` and others detailed below, avoids the need for unsafe blocks and fails gracefully if start and end indices are out of range. However, the two crates should not be added to the same project. If only need the core **substring** method and already the other well-supported crate, do not install this crate. On the other hand, if you need some of extra features available from this crate, uninstall the other crate before installing this one and replace ```use substring::*;``` with ```use substring_replace::*;```.

Regular Rust prefers *string slices* to manipulate strings by byte index ranges. However, it panics when byte indices are out of range or fall between character boundaries. Character indices are more intuitive and compatible with the popular [Regex](https://crates.io/crates/regex) crate. 

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

This method returns the end of a string (```&str``` or ```string```) from the specified start character index.
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
This method returns the remainder after removing a substring from a start index for n characters to the right or left.
It's the oposite to **substring_offset(position, length)**.
As with **substring_offset**, a negative length in the second parameter will will end at the reference index.
```rust
let sample_str = "indian-elephant";
let result = sample_str.substring_offset(7, 3);
// result will be "ele"
let result = sample_str.substring_offset(6, -3);
// result will be "ian"
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

### char_find

This finds the first character index of a plain string pattern. Like the standard *find* method, it returns an optional unsigned integer (usize). To search from right to left, but still returning the index of the first character in the matched sequence, you can use ```char_rfind```, 
```rust
let greek_words = "μήλα και πορτοκάλια";
let search_word = "και";
let character_index = greek_words.char_find(search_word);
let byte_index = greek_words.find(search_word);
println!("The word {search_word} starts at a character index of {character_index} and a byte index of {byte_index}");
// The word $search_word starts at a character index of 5 and a byte index of 9
```

---

NB: This is an alpha release, but the crate is feature-complete and supplements [string-patterns](https://crates.io/crates/string-patterns) and [simple-string-patterns](https://crates.io/crates/simple-string-patterns) .

### Version history

**1.3:** Added new methods ```.substring_remove(start: usize, end: usize)``` and ```.substring_pull(position: usize, length: i32)```.
**1.5:** Added new methods ```.char_find(pat: &str)``` and ```.char_rfind(pat: &str)```.