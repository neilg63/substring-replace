pub use to_offset::*;

/// Trait with extension methods to manipulate substrings by character indices
/// behaving like similar methods in other languages
pub trait SubstringReplace where Self:ToString {

    /// Return a substring by start and end character index
    /// With multibyte characters this will not be the same as the byte indices
    /// used by str slices
    fn substring<T: ToOffset>(&self, start: usize, end: T) -> &str;

    /// Return a substring from the start and to a specified end character index
    fn substring_start(&self, end: i64) -> &str {
        let end_index = if end < 0 { self.char_len().checked_sub(end.abs() as usize).unwrap_or(0) } else { end as usize };
        self.substring(0, end_index)
    }

    /// Return a substring from a specified start character index to a specified end
    /// A negative offset represents character index from the end, e.g. if character length is 15, -5 translates to 10
    /// If start index is greater than the max character index, the function will yield an empty string
    fn substring_end(&self, start: i64) -> &str {
        let max_index = self.char_len();
        let start_index = if start < 0 { max_index.checked_sub(start.abs() as usize).unwrap_or(0) } else { start as usize };
        self.substring(start_index, max_index)
    }

    /// Return a substring by start and end character index
    /// Unlike the default substring() method, the end index may be negative,
    /// in which case it counts backwards from the end, e.g. if character length is 15, -5 translates to 10
    #[deprecated(since = "0.2.1", note = "Use `substring` instead")]
    fn substring_range(&self, start: usize, end: i64) -> &str {
        self.substring(start, end)
    }


    // Replace substring delimited by start and end character index
    // with any string (&str)
    // To inject a string use substring_insert()
    fn substring_replace<T: ToOffset>(&self, replacement: &str, start: usize, end: T) -> String;

    /// Replace substring delimited by start and end character index
    /// Unlike the default substring_replace() method, the end index may be negative,
    /// in which case it counts backwards from the end, e.g. if character length is 15, -5 translates to 10
    #[deprecated(since = "0.2.1", note = "Use `substring` instead")]
    fn substring_replace_range(&self, replacement: &str, start: usize, end: i64) -> String {
        self.substring_replace(replacement, start, end)
    }


    /// Replace the start of a string to specified end character index
    /// e.g. "brown".substring_replace_start("d", 2);
    /// will replace the first two characters with "d", yield "down"
    /// A negative offset represents character index from the end, e.g. if character length is 15, -5 translates to 10
    fn substring_replace_start(&self, replacement: &str, end: i64) -> String {
        let end_index = if end < 0 { self.char_len().saturating_sub(end.abs() as usize) } else { end as usize };
        self.substring_replace(replacement, 0, end_index)
    }

    /// Replace the remainder of string from a specified start character index
    /// e.g. "blue".substring_replace_last("ack", 2);
    /// will replace the last 2 characters with "ack", yielding "black"
    /// A negative offset represents character index from the end, e.g. if character length is 15, -5 translates to 10
    fn substring_replace_end(&self, replacement: &str, start: i64) -> String {
        let end = self.char_len();
        let start_index = if start < 0 { end.saturating_sub(start.abs() as usize) } else { start as usize };
        self.substring_replace(replacement, start_index, end)
    }

    /// Remove a string delimited by a start and end character index
    /// e.g. "abcde".substring_remove(2, 4);
    /// will remove characters with indices of 2 and 3 (3rd and 4th or c and d)
    /// resulting in "abe", i.e. the opposite behaviour to substring()
    fn substring_remove(&self, start: usize, end: usize) -> String {
        self.substring_replace("", start, end)
    }

    /// Extract a substring from a start index for n characters to the right
    /// A negative length in the second parameter will start at the start index
    fn substring_offset(&self, position: usize, length: i32) -> &str {
        let (start, end) = position_and_offset_to_start_end(position, length);
        self.substring(start, end)
    }

    /// Remove a string from a start position to given length
    /// negative lengths will remove characters to the left
    /// e.g. "abcde".substring_remove(3, -3);
    /// will remove characters with indices of 1 and 2 (2nd and 3rd or b and c)
    /// resulting in "ade", i.e. the opposite behaviour to substring_offset()
    fn substring_pull(&self, position: usize, length: i32) -> String {
        let (start, end) = position_and_offset_to_start_end(position, length);
        self.substring_replace("", start, end)
    }

    /// Insert a string at a given character index
    /// This differs from String::insert by using character rather than byte indices
    /// to work better with multibyte characters
    /// It's also implemented for &str, while returning a new owned string
    fn substring_insert(&self, replacement: &str, start: usize) -> String {
        self.substring_replace(replacement, start, start)
    }

    /// Convert character index to start byte index
    fn to_start_byte_index(&self, start: usize) -> usize;

    /// Convert character index to end byte index
    fn to_end_byte_index(&self, start: usize) -> usize;

    /// Return the character length rather than the byte length
    fn char_len(&self) -> usize;

    /// Return the character index rather than the byte index of the first match of a pattern
    fn char_find(&self, pat: &str) -> Option<usize>;

    /// Return the character index rather than the byte index of the last match of a pattern
    /// this will be first index of the match
    fn char_rfind(&self, pat: &str) -> Option<usize>;

    /// Insert before or after the first or last occurrence
    fn insert_adjacent(&self, insert: &str, pat: &str, before: bool, first: bool) -> String;

    /// Insert before the first occurrence of a string
    fn insert_before_first(&self, insert: &str, pat: &str) -> String {
        self.insert_adjacent(insert, pat, true, true)
    }

    /// Insert before the first occurrence of a string
    fn insert_before_last(&self, insert: &str, pat: &str) -> String {
        self.insert_adjacent(insert, pat, true, false)
    }

    /// Insert after the last occurrence of a string
    fn insert_after_first(&self, insert: &str, pat: &str) -> String {
        self.insert_adjacent(insert, pat, false, true)
    }

    /// Insert after the last occurrence of a string
    fn insert_after_last(&self, insert: &str, pat: &str) -> String {
        self.insert_adjacent(insert, pat, false, false)
    }

    /// Insert between the first occurrence of a one string and the last occurrence of another
    fn insert_between(&self, insert: &str, start_pat: &str, end_pat: &str) -> String {
        if let Some(start_index) = self.char_find(start_pat) {
            if let Some(end_index) = self.char_rfind(end_pat) {
                return self.substring_replace(insert, start_index + 1, end_index);
            }
        }
        self.to_string()
    }

    /// Insert between the first occurrence of a one string and the last occurrence of another
    fn prepend(&self, insert: &str) -> String {
        [insert.to_string(), self.to_string()].concat()
    }

    fn append(&self, insert: &str) -> String {
        [self.to_string(), insert.to_string()].concat()
    }

}

impl SubstringReplace for str {

    /// Extract substring by character indices and hand overflow gracefully
    /// if the end index is equal or greater than start index, the function will yield an empty string 
    fn substring<T: ToOffset>(&self, start: usize, end: T) -> &str {
        let end_index = end.to_offset(self.char_len());
        if end_index > start {
            &self[self.to_start_byte_index(start)..self.to_end_byte_index(end_index)]
        } else {
            ""
        }
    }

    /// Replace a segment delimited by start and end characters indices with a string pattern (&str)
    fn substring_replace<T: ToOffset>(&self, replacement: &str, start: usize, end: T) -> String {
        let end_index = end.to_offset(self.char_len());
        [&self[0..self.to_start_byte_index(start)], replacement, &self[self.to_end_byte_index(end_index)..]].concat()
    }

    /// Translate the character start index to the start byte index
    /// to avoid boundary collisions with multibyte characters
    fn to_start_byte_index(&self, start: usize) -> usize {
        char_index_to_byte_index(self, start, false)
    }

    /// Translate the character end index to the end byte index
    /// to avoid boundary collisions with multibyte characters
    fn to_end_byte_index(&self, end: usize) -> usize {
        char_index_to_byte_index(self, end, true)
    }

    /// Return the character length as opposed to the byte length
    /// This will differ from len() only multibyte characters
    fn char_len(&self) -> usize {
        self.char_indices().count()
    }

    /// Return the character index of the first match of a given pattern
    fn char_find(&self, pat: &str) -> Option<usize>{
        extract_char_index(self, pat, false)
    }

    /// Return the character index rather than the byte index of the last match of a pattern
    /// this will be first index of the match
    fn char_rfind(&self, pat: &str) -> Option<usize>{
        extract_char_index(self, pat, true)
    }

    /// Insert before or after the first or last occurrence
    fn insert_adjacent(&self, insert: &str, pat: &str, before: bool, first: bool) -> String {
        if let Some(index) = extract_char_index(self, pat, !first) {
            let rel_index = if before {
                index
            } else {
                index + 1
            };
            self.substring_insert(insert, rel_index)
        } else {
            self.to_string()
        }
    }
}

/*
* private function to convert a character index to byte index requied by &str slices
*/
fn char_index_to_byte_index(text: &str, char_index: usize, to_end: bool) -> usize {
    let default_index = if to_end { text.len() } else { 0 };
    text.char_indices().nth(char_index).map(|(i, _)| i).unwrap_or(default_index)
}

/*
* private function to convert an index position and i32 position or negative offset length
* to valid start and end indices
* where the start must be positive and the end may not be before the start 
*/
fn position_and_offset_to_start_end(position: usize, length: i32) -> (usize, usize) {
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
    (start, end)
}

/// private function to extract the character index of pattenr (char sequence)
fn extract_char_index(text: &str, pat: &str, reverse: bool) -> Option<usize> {
    let mut start_index: Option<usize> = None;
    let pat_chars = pat.chars().collect::<Vec<_>>();
    let pat_len = pat.char_len();
    let text_chars = text.chars().collect::<Vec<_>>();
    let num_text_chars = text_chars.len();
    let range = 0..num_text_chars;
    let mut next_pat_char_index = if reverse { pat_len - 1 } else { 0 };
    let mut temp_pat_len = 0;
    for tc_index in range {
        let rel_index = if reverse { num_text_chars - 1 - tc_index } else { tc_index };
        let tc = text_chars[rel_index];
        if tc == pat_chars[next_pat_char_index] {
            if !reverse && next_pat_char_index == 0 {
                start_index = Some(rel_index);
            }
            if pat_len > 1 {
                if reverse {
                    if next_pat_char_index > 0 {
                        next_pat_char_index -= 1;
                    }
                } else {
                    next_pat_char_index += 1;
                }
            }
            temp_pat_len += 1;
        } else {
            next_pat_char_index = if reverse { pat_len - 1 } else { 0 };
            temp_pat_len = 0;
        }
        if temp_pat_len == pat_len {
            if reverse {
                start_index = Some(rel_index);
            }
            break;
        }
    }
    start_index
}
