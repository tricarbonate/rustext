use termion::event::{Event, Key};

pub mod modes;
// use modes::EditorMode;
use crate::utils::*;
use crate::renderer::Renderer;
use crate::renderer::types::*;
use crate::renderer::buffer::Buffer;
use crate::renderer::cursor::*;
use crate::renderer::status::{Status, EditorMode};

pub struct CommandHandler {}

impl CommandHandler {
    pub fn default() -> Self {
        Self {}
    }

    pub fn handle(&mut self,
        event: Option<Event>, 
        renderer: &mut Renderer, 
        buf: &mut Buffer,
        cursor: &mut Cursor,
        status: &mut Status
    ) {
        match event {
            None => return,
            Some(e) => {
                match e {
                    Event::Key(_c) => self.handle_key(_c, renderer, buf, cursor, status),
                    _ => println!("Unsupported event {:?}", e),
                }
            }
        }
    }

    fn handle_key(&mut self,
        key: Key,
        renderer: &mut Renderer,
        buf: &mut Buffer,
        cursor: &mut Cursor,
        status: &mut Status
    ) {
        match status.mode() {
            EditorMode::Insert => {
                match key {
                    Key::Char(_c) => { 
                        let p = cursor.pos();
                        buf.insert(_c, p.y - 1, p.x - 1) ;
                        cursor.move_cursor(Direction::Right, buf);
                    },
                    Key::Backspace => { 
                        let p = cursor.pos();
                        buf.backspace(p.y - 1, p.x - 1) ;
                        cursor.move_cursor(Direction::Left, buf);
                    },
                    Key::Esc => { 
                        status.set_mode(EditorMode::Normal);
                        print!("{}", termion::cursor::SteadyBlock);
                    },
                    _ => return,
                }
            },

            EditorMode::Normal => {
                match key {
                    Key::Char('i') => {
                        status.set_mode(EditorMode::Insert);
                        print!("{}", termion::cursor::BlinkingBar);
                    },
                    Key::Char('o') => {
                        status.set_mode(EditorMode::Insert);
                        buf.insert_row(cursor.pos().y);
                        cursor.move_cursor(Direction::Down, buf);
                        print!("{}", termion::cursor::BlinkingBar);
                    },
                    Key::Char('s') => {
                        buf.save();
                        status.set_message(String::from("Saved"));
                    }
                    Key::Esc => status.set_mode(EditorMode::Normal), 
                    Key::Char('q') => die(None),
                    Key::Char('h') => cursor.move_cursor(Direction::Left, buf),
                    Key::Char('j') => cursor.move_cursor(Direction::Down, buf),
                    Key::Char('k') => cursor.move_cursor(Direction::Up, buf),
                    Key::Char('l') => cursor.move_cursor(Direction::Right, buf),
                    _ => return,
                }
            },

            EditorMode::Command => {
            },
        }
    }
}
