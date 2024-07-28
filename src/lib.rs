
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

    /// Extract a substring from a start index for n characters to the right
    /// A negative length in the second parameter will start at the start index
    fn substring_offset(&self, position: usize, length: i32) -> &str {
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
        self.substring(start, end)
    }

    /// Insert a string at a given character index
    /// This differs from String::insert as it uses character rather than byte indices
    /// and thus works better with multibyte characters
    /// It's also implemented to str, while returning a new owned string
    fn substring_insert(&self, replacement: &str, start: usize) -> String {
        self.substring_replace(replacement, start, start)
    }

    /// Convert character index to start byte index
    fn to_start_byte_index(&self, start: usize) -> usize;

    /// Convert character index to end byte index
    fn to_end_byte_index(&self, start: usize) -> usize;

    /// Return the character length rather than the byte length
    fn char_len(&self) -> usize;

}

impl SubstringReplace for str {

    /// Extract substring by character indices and hand overflow gracefully
    /// if the end index is equal or greater than start index, the function will yield an empty string 
    fn substring(&self, start: usize, end: usize) -> &str {
        let end_index = if end > start { end } else { start };
        &self[self.to_start_byte_index(start)..self.to_end_byte_index(end_index)]
    }

    /// Replace 
    fn substring_replace(&self, replacement: &str, start: usize, end: usize) -> String {
        let end_index = if end > start { end } else { start };
        [&self[0..self.to_start_byte_index(start)], replacement, &self[self.to_end_byte_index(end_index)..]].concat()
    }

    /// Translate the character start index to the start byte index
    /// to avoid boundary collisions with multibyte characters
    fn to_start_byte_index(&self, start: usize) -> usize {
        self.char_indices().nth(start).map(|(i, _)| i).unwrap_or(0)
    }

    /// Translate the character end index to the end byte index
    /// to avoid boundary collisions with multibyte characters
    fn to_end_byte_index(&self, end: usize) -> usize {
        self.char_indices().nth(end).map(|(i, _)| i).unwrap_or(self.len())
    }

    /// Return the character length as opposed to the byte length
    /// This will differ from len() only multibyte characters
    fn char_len(&self) -> usize {
        self.char_indices().count()
    }
}

