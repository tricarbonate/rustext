use crate::renderer::types::*;
use crate::renderer::buffer::Buffer;

use std::cmp;

const START_COL: usize = 3;
const START_ROW: usize = 1;

pub struct Cursor {
    pos: Position,
}

impl Cursor {
    pub fn default() -> Self {
        Self {
            pos: Position { x: START_COL, y: START_ROW },
        }
    }

    pub fn move_cursor(&mut self, dir: Direction, buf: &mut Buffer) {
        // let pos = &self.pos;

        let curr_row_len = buf.row_len(self.pos.y - 1);
        let buf_len = buf.len();

        match dir {
            Direction::Right => {
                self.pos.x = cmp::min(self.pos.x + 1, curr_row_len + START_COL - 1);
            },
            Direction::Left => {
                if self.pos.x <= START_COL { return; }
                self.pos.x -= 1;
            },
            Direction::Down => {
                self.pos.y = cmp::min(self.pos.y + 1, buf_len);
                let curr_row_len = buf.row_len(self.pos.y - 1);
                self.pos.x = cmp::min(self.pos.x, curr_row_len + START_COL - 1);
            },
            Direction::Up => {
                if self.pos.y <= START_ROW { return; }
                self.pos.y -= 1;
                let curr_row_len = buf.row_len(self.pos.y - 1);
                self.pos.x = cmp::min(self.pos.x, curr_row_len + START_COL - 1);
            },
        }
    }

    pub fn set_cursor(&mut self, x: usize, y: usize) {
        self.pos = Position { x, y };
    }

    pub fn pos(&self) -> &Position {
        &self.pos
    }
}
