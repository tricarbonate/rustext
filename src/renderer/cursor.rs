use crate::renderer::types::*;
use crate::renderer::buffer::Buffer;

use std::cmp;

pub const START_COL: usize = 3;
pub const START_ROW: usize = 1;

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
        let size = match termion::terminal_size() {
            Ok(s) => s,
            Err(e) => {(0, 0)}
        };
        let screen_width = size.0;
        let screen_height = size.1;

        let curr_row_len = buf.row_len(self.pos.y - 1);
        let buf_len = buf.len();

        match dir {
            Direction::Right => {
                self.pos.x += 1; //cmp::min(self.pos.x + 1, curr_row_len + START_COL - 1);
                self.clip_cursor(buf);
            },
            Direction::Left => {
                if self.pos.x <= START_COL { return; }
                self.pos.x -= 1;
                self.clip_cursor(buf);
            },
            Direction::Down => {
                if self.pos.y + buf.scroll.y >= buf_len {
                    return;
                }
                if self.pos.y >= (size.1 as usize - 2) {
                    buf.scroll.y += 1;
                } else {
                    self.pos.y += 1;
                }

                self.clip_cursor(buf);
            },
            Direction::Up => {
                if self.pos.y <= START_ROW && buf.scroll.y == 0 { 
                    return; 
                }

                if self.pos.y <= START_ROW { 
                    buf.scroll.y -= 1;
                } else {
                    self.pos.y -= 1;
                }

                self.clip_cursor(buf);
            },
        }
    }

    /*
     * Sets the cursor position to the current row's last character,
     * or to the first cell when the row is empty
     */
    pub fn clip_cursor(&mut self, buf: &Buffer) {
        let curr_row_len = buf.row_len(self.pos.y + buf.scroll.y - 1);
        if curr_row_len == 0 {
            self.pos.x = START_COL;
        } else {
            self.pos.x = cmp::min(self.pos.x, curr_row_len + START_COL - 1);
        }
    }

    // Moves the cursor in insert mode, without clipping operations
    pub fn insert_move(&mut self) {
        self.pos.x += 1; 
    }
    pub fn backspace_move(&mut self) {
        if self.pos.x <= START_COL { return; }
        self.pos.x -= 1;
    }

    pub fn set_cursor(&mut self, x: usize, y: usize) {
        self.pos = Position { x, y };
    }

    pub fn pos(&self) -> &Position {
        &self.pos
    }
}
