use termion::raw::{IntoRawMode, RawTerminal};
use std::io::{self, stdout, Write};
use std::cmp;

pub mod buffer;
use buffer::Buffer;

pub struct Size {
    pub width: usize,
    pub height: usize,
}

pub struct Position {
    x: usize,
    y: usize,
}

pub enum Direction {
    Right,
    Left,
    Up,
    Down
}

pub struct Renderer {
    size: Size,
    cursor_pos: Position,
    _stdout: RawTerminal<std::io::Stdout>,
}

impl Renderer {
    pub fn default() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size {
                width: size.0 as usize,
                height: size.1 as usize,
            },
            cursor_pos: Position { x: 1, y: 1 },
            _stdout: stdout().into_raw_mode()?,
        })
    }

    pub fn init(&mut self) {
        self.clear_screen();
        self.move_cursor_xy(1, 1);
        self.draw_rows();
        self.move_cursor_xy(3, 1);
    }

    pub fn draw_char(&mut self, c: char) -> Result<(), std::io::Error> {
        print!("{}", c);
        // self.cursor_pos.x += 1;
        self.flush()
    }

    pub fn draw_rows(&self) {
        for _ in 0..self.size.height {
            println!("~\r");
        }
    }

    pub fn draw_buffer(&mut self, buffer: &Buffer) {
        for i in 0..buffer.len() {
            print!("{}", termion::clear::CurrentLine);
            print!("~ ");
            for c in buffer.rows[i].string.chars() {
                self.draw_char(c).ok();
            }
            println!("\r");
        }

        // keep drawing tildes ~~~
        for i in buffer.len()+1..self.size.height {
            print!("{}", termion::clear::CurrentLine);
            println!("~\r");
        }
    }

    pub fn modify_buffer(&mut self, c: char, buffer: &mut Buffer) {
        let row_index = self.cursor_pos.y - 1;
        let col_index = self.cursor_pos.x - 1;
        match buffer.row(row_index) {
            None => {
                // todo: create empty lines
            },
            Some(row) => {
                row.string.insert(col_index - 2, c);
            }
        }
        self.cursor_pos.x += 1;
    }

    pub fn backspace_buffer(&mut self, buffer: &mut Buffer) {
        let row_index = self.cursor_pos.y - 1;
        let col_index = self.cursor_pos.x - 1;
        match buffer.row(row_index) {
            None => {
                // todo: create empty lines
            },
            Some(row) => {
                // row.string.(col_index - 2, c);
                row.string.remove(col_index - 3);
            }
        }
        self.cursor_pos.x -= 1;
    }

    pub fn move_cursor(&mut self, dir: Direction) {
        let pos = &self.cursor_pos;
        match dir {
            Direction::Right => {
                // self.move_cursor_xy(cmp::min(pos.x + 1, self.size.width), pos.y)
                self.cursor_pos.x += 1;
            },
            Direction::Left => {
                if pos.x == 0 { return; }
                // self.move_cursor_xy(cmp::max(pos.x - 1, 1), pos.y)
                self.cursor_pos.x -= 1;
            },
            Direction::Down => {
                // self.move_cursor_xy(pos.x, cmp::min(pos.y + 1, self.size.height))
                self.cursor_pos.y += 1;
            },
            Direction::Up => {
                if pos.y == 0 { return; }
                self.cursor_pos.y -= 1;
                // self.move_cursor_xy(pos.x, cmp::max(pos.y - 1, 1))
            },
        }
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen(&self) {
        print!("{}", termion::clear::All);
    }

    pub fn refresh_screen(&mut self, buffer: &Buffer) {
        self.clear_screen();
        self.move_cursor_xy(1, 1);
        self.draw_buffer(buffer);
        self.move_cursor_xy(self.cursor_pos.x, self.cursor_pos.y);
        // self.move_cursor_xy(1, 1);
        self.flush();
    }

    pub fn move_cursor_xy(&mut self, x: usize, y: usize) {
        // self.cursor_pos = Position { x, y };
        print!("{}", termion::cursor::Goto(x as u16, y as u16));
        self.flush();
    }

    pub fn flush(&self) -> Result<(), std::io::Error> {
        io::stdout().flush()
    }
}
