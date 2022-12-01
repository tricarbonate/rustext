use crate::renderer::types::*;

struct Insertion {
    buffer: String,
    start: Position,
    content: String,
}

pub struct InsertHistory {
    insert_history: Vec<Insertion>,
    pointer: usize,
}

impl InsertHistory {
    pub fn default() -> Self {
        Self { 
            insert_history: Vec::new(),
            pointer: 0,
        }
    }

    // pub fn push_insert_hist
}
