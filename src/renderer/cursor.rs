use crate::renderer::types::*;

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

    pub fn move_cursor(&mut self, dir: Direction) {
        let pos = &self.pos;
        match dir {
            Direction::Right => {
                self.pos.x += 1;
            },
            Direction::Left => {
                if pos.x <= START_COL { return; }
                self.pos.x -= 1;
            },
            Direction::Down => {
                self.pos.y += 1;
            },
            Direction::Up => {
                if pos.y <= START_ROW { return; }
                self.pos.y -= 1;
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
