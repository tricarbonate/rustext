use std::fs;
use std::io::{Error, Write};

use crate::renderer::types::*;
use crate::buffer::buffer_row::Row;
use crate::buffer::insert_history::InsertHistory;

pub const START_COL: usize = 7;
pub const START_ROW: usize = 1;

#[derive(Default)]
pub struct Buffer {
    pub rows: Vec<Row>,
    pub name: String,
    pub scroll: Position,
    pub saved: bool,
    pub hist: InsertHistory,
}

impl Buffer {
    pub fn default() -> Self {
        let mut rows = Vec::new();
        rows.push(Row::from(" "));
        Self {
            rows,
            name: String::from("unnamed"),
            scroll: Position { x: 0, y: 0 },
            saved: true,
            hist: InsertHistory::default()
        }
    }

    pub fn from_file(filename: String) -> Self {
        let contents = fs::read_to_string(&filename).unwrap();
        let mut rows = Vec::new();
        for value in contents.lines() {
            rows.push(Row::from(value))
        }
        Self {
            rows,
            name: String::from(&filename),
            scroll: Position { x: 0, y: 0 },
            saved: true,
            hist: InsertHistory::default()
        }
    }

    pub fn load_from_file(&mut self, filename: String) {
        let contents = fs::read_to_string(&filename).unwrap();
        self.rows.clear();
        for value in contents.lines() {
            self.rows.push(Row::from(value))
        }
        self.name = filename;
        self.scroll = Position { x: 0, y: 0 };
    }

    pub fn save(&mut self, filename: Option<String>) {
        match filename {
            Some(new_filename) => self.name = new_filename,
            None => {}
        }
        let mut file = fs::File::create(&self.name).unwrap();
        for row in &self.rows {
            file.write_all(row.string.as_bytes()).unwrap();
            file.write_all(b"\n").unwrap();
        }
        self.saved = true;
    }

    pub fn backspace(&mut self, rowidx: usize, colidx: usize) {
        if rowidx >= self.len() {
            return;
        }
        
        if self.rows[rowidx].string.is_empty() {
            self.rows.remove(rowidx);
            return;
        } else if colidx < 3 {
            if rowidx > 0 {
                // let s = &self.rows[rowidx].string;
                // self.rows[rowidx-1].string += &s[..];
            }
            return;
        }
        self.rows[rowidx].string.remove(colidx-3);
        self.saved = false;
    }

    pub fn insert(&mut self, c: char, row: usize, col: usize) {
        match self.row(row + self.scroll.y) {
            None => {},
            Some(row) => {
                row.string.insert(col - (START_COL - 1), c);
            }
        }
        self.saved = false;
    }

    pub fn insert_row(&mut self, row: usize) {
        self.rows.insert(row + self.scroll.y, Row::default());
        self.saved = false;
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

    pub fn hist_mut(&mut self) -> &mut InsertHistory {
        &mut self.hist
    }
}

