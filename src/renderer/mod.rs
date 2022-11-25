use termion::raw::{IntoRawMode, RawTerminal};
use std::io::{self, stdout, Write};
use std::cmp;

pub mod buffer;
pub mod types;
pub mod cursor;
pub mod status;
use buffer::Buffer;
use status::Status;
use types::*;

const START_COL: usize = 3;
const START_ROW: usize = 1;

pub struct Renderer {
    size: Size,
    // cursor_pos: Position,
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
            // cursor_pos: Position { x: START_COL, y: START_ROW },
            _stdout: stdout().into_raw_mode()?,
        })
    }

    pub fn draw_char(&mut self, c: char) -> Result<(), std::io::Error> {
        print!("{}", c);
        self.flush()
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

        // keep drawing tildes ~
        for i in buffer.len()+1..self.size.height - 1 {
            print!("{}", termion::clear::CurrentLine);
            println!("~\r");
        }
    }

    pub fn draw_status_bar(&mut self, status: &Status) {
        println!("{}", status.formatted());
    }

    pub fn reset_cursor(&self) {
        print!("{}", termion::cursor::Goto(1, 1));
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen(&self) {
        print!("{}", termion::clear::All);
    }

    pub fn refresh_screen(&mut self, buffer: &Buffer, status: &Status) {
        self.clear_screen();
        self.reset_cursor();
        self.draw_buffer(buffer);
        self.draw_status_bar(status);
    }

    pub fn flush(&self) -> Result<(), std::io::Error> {
        io::stdout().flush()
    }
}
