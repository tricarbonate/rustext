
use crate::renderer::types::*;

struct Insertion {
    start: Position,
    content: String,
}

impl Insertion {
    pub fn default() -> Self {
        Self {
            start: Position { x: 0, y: 0 },
            content: String::new()
        }
    }

    pub fn from(pos: Position) -> Self {

        Self {
            start: pos,
            content: String::new()
        }
    }
}

pub struct InsertHistory {
    insert_history: Vec<Insertion>,
    pointer: usize,
}

impl Default for InsertHistory {
    fn default() -> Self {
        Self { 
            insert_history: Vec::new(),
            pointer: 0,
        }
    }
}

impl InsertHistory {
    pub fn start_entry(&mut self, pos: Position) {
        self.insert_history.push(Insertion::from(pos));
        self.pointer += 1;
    }

    pub fn insert(&mut self, c: char) {
        // self.insert_history[self.insert_history.len() - 1].content.push(c);
    }

    pub fn backspace(&mut self) {
        // if self.insert_history[self.insert_history.len() - 1].content.is_empty() { 
        //     return;
        // }
        // self.insert_history[self.insert_history.len() - 1].content.remove()
    }
}
