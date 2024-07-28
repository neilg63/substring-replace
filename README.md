[![mirror](https://img.shields.io/badge/mirror-github-blue)](https://github.com/neilg63/substring-replace)
[![crates.io](https://img.shields.io/crates/v/substring-replace.svg)](https://crates.io/crates/substring-replace)
[![docs.rs](https://docs.rs/substring-replace/badge.svg)](https://docs.rs/substring-replace)

# substring-replace: Extract, insert and replace substrings

This crate adds a set of convenient methods to easily extract, insert and replace string slices in Rust with character indices compatibile with multibyte characters.

Do not add this library to your project if it already depends on the [substring](https://crates.io/crates/substring) crate. Its core substring method, while sharing the same signature and functionality, will conflict with the same method in the ```SubstringReplace``` trait, although the implementation in this crate avoids an unsafe block and will not panic if the start and end indices are out of range.

Regular Rust prefers ```str``` slices