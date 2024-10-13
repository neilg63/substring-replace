use substring_replace::*;

#[cfg(test)]

/// Test substring works as expected with &str and string types
#[test]
fn test_extract_substring() {
    let sample_str = "We can't solve today's problems.";
    assert_eq!(sample_str.substring(9, 14), "solve");

    assert_eq!(sample_str.to_string().substring(9, 14), "solve");
}

/// Test substring works as expected with &str and string types
#[test]
fn test_negative_offset() {
  let sample_str = "2024-10-12T17:32:43.00Z";
  let expected_result = "10-12T17:32:43";
  
  assert_eq!(sample_str.substring(5,-4), expected_result);
}
 
/// Test substring fails gracefully with out-of-range start or end indices
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

/// Test substring_replace with single-byte characters
#[test]
fn test_replace_substring() {
  let sample_str = "We can't solve today's problems.";
  let target_str = "We cannot solve today's problems.";
  assert_eq!(sample_str.substring_replace("cannot", 3, 8), target_str);
}

/// Test substring_replace with multibyte characters
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

  let date_str = "2024-09-15T14:43:32.123Z";
  let repl = "15:17:54";
  let target_str = "2024-09-15T15:17:54.123Z";
  
  assert_eq!(date_str.substring_replace(repl, 11, -5), target_str);
}

/// Test substring_insert works correctly with character indices
#[test]
fn test_inject_substring() {
  let sample_str_1 = "adefg";
  let target_str_1 = "abcdefg";
  assert_eq!(sample_str_1.substring_insert("bc", 1), target_str_1);

  let sample_str_2 = "a🐫defg";
  let target_str_2 = "a🐫bcdefg";
  assert_eq!(sample_str_2.substring_insert("bc", 2), target_str_2);
}

/// Test substring_remove and substring_pull remove the correct characters
#[test]
fn test_substring_remove() {
  let sample_str = "abcdefg";
  assert_eq!(sample_str.substring_remove(2, 5), "abfg");
  // remove 2 characters before index 3
  assert_eq!(sample_str.substring_pull(3, -2), "adefg");
  // remove 2 characters from index 3
  assert_eq!(sample_str.substring_pull(3, 2), "abcfg");

  let sample_str = "three-horse-race";
  let result = sample_str.substring_pull(5, 6);
  assert_eq!(result, "three-race");
}


#[test]
fn test_substring_replace_start_end() {
  let sample_str = "abcdefg";
  let extra = "xyz";
  assert_eq!(sample_str.substring_replace_start(extra, 3), "xyzdefg");
  assert_eq!(sample_str.substring_replace_end(extra, 3), "abcxyz");
}

/// Test char_len() with a range of multibyte characters
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

/// Test char_find and char_rfind with a range of multibyte characters
#[test]
fn test_character_find_index() {
  // Devanagari characters use 3 bytes each
  let sample_str = "नमस्ते";
  assert_eq!(sample_str.char_find("म"), Some(1));

  let sample_str = "a🐕cd🐕fg";
  assert_eq!(sample_str.char_rfind("🐕"), Some(4));

  let sample_str = "আমরা সবজি দিয়ে ভাত রান্না করেছি";
  assert_eq!(sample_str.char_rfind("ভাত"), Some(16));

  let sample_str = "abc123abc123";
  assert_eq!(sample_str.char_rfind("abc"), Some(6));

  let greek_words = "μήλα και πορτοκάλια";
  let search_word = "και";
  let character_index = greek_words.char_find(search_word).unwrap_or(0);
  let byte_index = greek_words.find(search_word).unwrap_or(0);
  assert_eq!(character_index, 5);
  assert_eq!(byte_index, 9);
}


#[test]
fn test_substring_start_end() {
  let sample_str = "/long/file/path";
  let result = sample_str.substring_end(5);
  assert_eq!(result, "/file/path");
  let result = sample_str.substring_end(-4);
  assert_eq!(result, "path");
  let result = sample_str.substring_start(-4);
  assert_eq!(result, "/long/file/");
  // alows negative offset
  let result = sample_str.substring(5,-5);
  assert_eq!(result, "/file");
}


#[test]
fn test_substring_insert_adjacent() {
  let sample_str = "long-file-name.revised.jpg";
  let result = sample_str.insert_before_last("-123", ".");
  let target_str = "long-file-name.revised-123.jpg";
  assert_eq!(result, target_str);
  let result = sample_str.insert_after_first("document-", "-");
  let target_str = "long-document-file-name.revised.jpg";
  assert_eq!(result, target_str);
  let result = sample_str.insert_between("document", "-", ".");
  let target_str = "long-document.jpg";
  assert_eq!(result, target_str);
}
 