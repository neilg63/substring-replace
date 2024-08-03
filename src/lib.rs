/// Trait with extension methods to manipulate substrings by character indices
/// behaving like similar methods in other languages
pub trait SubstringReplace {

    /// Return a substring by start and end character index
    /// With multibyte characters this will not be the same as the byte indices
    /// used by str slices
    fn substring(&self, start: usize, end: usize) -> &str;

    /// Return a substring from the start and to a specified end character index
    fn substring_start(&self, end: usize) -> &str {
        self.substring(0, end)
    }

    /// Return a substring from a specified start character index to a specified end
    /// If start index is greater than the max character index, the function will yield an empty string
    fn substring_end(&self, start: usize) -> &str {
        let max_index = self.char_len();
        self.substring(start, max_index)
    }

    // Replace substring delimited by start and end character index
    // with any string (&str)
    // To inject a string use substring_insert()
    fn substring_replace(&self, replacement: &str, start: usize, end: usize) -> String;


    /// Replace the start of a string to specified end character index
    /// e.g. "brown".substring_replace_start("d", 2);
    /// will replace the first two characters with "d", yield "down"
    fn substring_replace_start(&self, replacement: &str, end: usize) -> String {
        self.substring_replace(replacement, 0, end)
    }

    /// Replace the remainder of string from a specified start character index
    /// e.g. "blue".substring_replace_last("ack", 2);
    /// will replace the last 2 characters with "ack", yielding "black"
    fn substring_replace_end(&self, replacement: &str, start: usize) -> String {
        let end = self.char_len();
        self.substring_replace(replacement, start, end)
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

}

impl SubstringReplace for str {

    /// Extract substring by character indices and hand overflow gracefully
    /// if the end index is equal or greater than start index, the function will yield an empty string 
    fn substring(&self, start: usize, end: usize) -> &str {
        let end_index = if end > start { end } else { start };
        &self[self.to_start_byte_index(start)..self.to_end_byte_index(end_index)]
    }

    /// Replace a segment delimited by start and end characters indices with a string pattern (&str)
    fn substring_replace(&self, replacement: &str, start: usize, end: usize) -> String {
        let end_index = if end > start { end } else { start };
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
