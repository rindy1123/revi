use std::cmp;
use unicode_segmentation::UnicodeSegmentation;

#[derive(Default)]
pub struct Row {
    pub string: String,
    len: usize,
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        Self {
            string: String::from(slice),
            len: slice.graphemes(true).count(),
        }
    }
}

impl Row {
    pub fn len(&self) -> usize {
        self.len
    }

    /// Index is zero-based
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        self.string
            .graphemes(true)
            .skip(start)
            .take(end - start)
            .collect()
    }

    pub fn insert(&mut self, at: usize, c: char) {
        let mut string = String::new();
        let mut length = 0;
        for (index, grapheme) in self.string.graphemes(true).enumerate() {
            if index == at {
                length += 1;
                string.push(c);
            }
            length += 1;
            string.push_str(grapheme);
        }
        self.string = string;
        self.len = length;
    }
}
