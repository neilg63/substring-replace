use substring_replace::*;

#[cfg(test)]

#[test]
fn test_extract_substring() {
    let sample_str = "We can't solve today's problems.";
    assert_eq!(sample_str.substring(9, 14), "solve");

    assert_eq!(sample_str.to_string().substring(9, 14), "solve");
}

#[test]
fn test_extract_substring_overflow() {
    let sample_str = "Thinking is hard work.";
    assert_eq!(sample_str.substring(9, 50), "is hard work.");

    assert_eq!(sample_str.substring(9, 3), "");
}

#[test]
fn test_extract_substring_offset() {
    let sample_str = "The strangest tale that ever I heard";
    // extract with exact character indices 
    assert_eq!(sample_str.substring(4, 13), "strangest");

    // We know the sample word starts at index 14 need the next 4 characters
    assert_eq!(sample_str.substring_offset(14, 4), "tale");

    // We know the sample position is 13 and need the previous 3 characters
    assert_eq!(sample_str.substring_offset(13, -3), "est");
}

#[test]
fn test_replace_substring() {
  let sample_str = "We can't solve today's problems.";
  let target_str = "We cannot solve today's problems.";
  assert_eq!(sample_str.substring_replace("cannot", 3, 8), target_str);
}

#[test]
fn test_replace_substring_multibyte() {
  let str_ar = "مرحبًا أحمد";
  let repl = "سامي";
  
  let target_str = "مرحبًا سامي";
  assert_eq!(str_ar.substring_replace(repl, 7, 11), target_str);
  let str_hi = "नीली साड़ी";
  
  let repl = "नीली";
  let target_str = "नीली साड़ी";
  assert_eq!(str_hi.substring_replace(repl, 0, 4), target_str);
}

#[test]
fn test_inject_substring() {
  let sample_str = "adefg";
  let target_str = "abcdefg";
  assert_eq!(sample_str.substring_insert("bc", 1), target_str);
}

#[test]
fn test_substring_remove() {
  let sample_str = "abcdefg";
  assert_eq!(sample_str.substring_remove(2, 5), "abfg");
  // remove 2 characters before index 3
  assert_eq!(sample_str.substring_pull(3, -2), "adefg");
  // remove 2 characters from index 3
  assert_eq!(sample_str.substring_pull(3, 2), "abcfg");
}


#[test]
fn test_substring_replace_start_end() {
  let sample_str = "abcdefg";
  let extra = "xyz";
  assert_eq!(sample_str.substring_replace_start(extra, 3), "xyzdefg");
  assert_eq!(sample_str.substring_replace_end(extra, 3), "abcxyz");
}

#[test]
fn test_character_length() {
  // Devanagari characters use 3 bytes each
  let sample_str = "नमस्ते";
  assert_eq!(sample_str.char_len(), sample_str.len() / 3);
  // Arabic characters use 2 bytes each
  let sample_str = "سلام";
  assert_eq!(sample_str.char_len(), sample_str.len() / 2);

  // Most emojis use 4 bytes each
  let sample_str = "😎";
  assert_eq!(sample_str.char_len(), sample_str.len() / 4);

  // Non ASCII letters used in Latin-derived 
  let sample_str = "œ";
  assert_eq!(sample_str.char_len(), sample_str.len() / 2);

  let sample_str = "à";
  assert_eq!(sample_str.char_len(), sample_str.len() / 2);

}
