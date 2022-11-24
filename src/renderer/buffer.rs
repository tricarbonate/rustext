
pub struct Row {
    pub string: String
}

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

    pub fn row(&mut self, index: usize) -> Option<&mut Row> {
        self.rows.get_mut(index)
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }
}

impl From<&str> for Row {
    fn from(slice: &str) -> Self {
        Self {
            string: String::from(slice),
        }
    }
}

// impl Row {
// }
