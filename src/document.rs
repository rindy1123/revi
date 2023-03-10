use std::fs;

use crate::{editor::Position, row::Row};

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
}

impl Document {
    pub fn open(file_name: &str) -> Result<Document, std::io::Error> {
        let contents = fs::read_to_string(file_name)?;
        let mut rows = Vec::new();
        for line in contents.lines() {
            rows.push(Row::from(line));
        }
        Ok(Self { rows })
    }

    pub fn row(&self, line_num: usize) -> Option<&Row> {
        self.rows.get(line_num)
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn insert(&mut self, at: &Position, c: char) {
        let Position { x, y } = at;
        if let Some(row) = self.rows.get_mut(*y) {
            // TODO: deal with new line
            if c != '\n' {
                row.insert(*x, c);
            }
        }
    }
}
