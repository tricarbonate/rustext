use std::fs;
use std::io::{Error, Write};

use crate::renderer::types::*;
use crate::renderer::cursor::*;

#[derive(Default)]
pub struct Buffer {
    pub rows: Vec<Row>,
    pub name: String,
    pub scroll: Position
}

impl Buffer {
    pub fn default() -> Self {
        Self {
            rows: Vec::new(),
            name: String::from("unnamed"),
            scroll: Position { x: 0, y: 0 },
        }
    }

    pub fn from_example() -> Self {
        let mut rows = Vec::new();
        rows.push(Row::from("Hello, World!"));
        rows.push(Row::from("Hello, World2!"));
        rows.push(Row::from("Hello, World3!"));
        Self { 
            rows, 
            name: String::from("new"),
            scroll: Position { x: 0, y: 0 },
        }
    }

    pub fn from_file(filename: String) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(&filename)?;
        let mut rows = Vec::new();
        for value in contents.lines() {
            rows.push(Row::from(value))
        }
        Ok(Self {
            rows,
            name: String::from(&filename),
            scroll: Position { x: 0, y: 0 },
        })
    }

    pub fn save(&mut self, filename: Option<String>) -> Result<(), Error> {
        match filename {
            Some(new_filename) => self.name = new_filename,
            None => {}
        }
        let mut file = fs::File::create(&self.name)?;
        for row in &self.rows {
            file.write_all(row.string.as_bytes())?;
            file.write_all(b"\n")?;
        }
        Ok(())
    }

    pub fn backspace(&mut self, row: usize, col: usize) {
        match self.row(row) {
            None => {},
            Some(row) => {
                if row.string.is_empty() || col < 3 { return }
                row.string.remove(col - 3);
            }
        }
    }

    pub fn insert(&mut self, c: char, row: usize, col: usize) {
        match self.row(row + self.scroll.y) {
            None => {},
            Some(row) => {
                row.string.insert(col - 2, c);
            }
        }
    }

    pub fn insert_row(&mut self, row: usize) {
        self.rows.insert(row + self.scroll.y, Row::default());
    }

    pub fn row_len(&self, index: usize) -> usize {
        match self.rows.get(index) {
            None => 0,
            Some(r) => r.string.len()
        }
    }

    pub fn row(&mut self, index: usize) -> Option<&mut Row> {
        self.rows.get_mut(index)
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

pub struct Row {
    pub string: String
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        Self {
            string: String::from(slice),
        }
    }
}

impl Row {
    pub fn default() -> Self {
        Self {
            string: String::new()
        }
    }
}
