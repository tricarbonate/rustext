#[derive(Default)]
pub struct Buffer {
    pub rows: Vec<Row>,
}

impl Buffer {
    pub fn default() -> Self {
        Self {
            rows: Vec::new()
        }
    }

    pub fn from_example() -> Self {
        let mut rows = Vec::new();
        rows.push(Row::from("Hello, World!"));
        rows.push(Row::from("Hello, World2!"));
        rows.push(Row::from("Hello, World3!"));
        Self { rows }
    }

    pub fn backspace(&mut self, row: usize, col: usize) {
        match self.row(row) {
            None => {},
            Some(row) => {
                row.string.remove(col - 3);
            }
        }
    }

    pub fn insert(&mut self, c: char, row: usize, col: usize) {
        match self.row(row) {
            None => {},
            Some(row) => {
                row.string.insert(col - 2, c);
            }
        }
    }

    pub fn insert_row(&mut self, row: usize) {
        self.rows.insert(row, Row::default());
    }

    pub fn row_len(&mut self, index: usize) -> usize {
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
