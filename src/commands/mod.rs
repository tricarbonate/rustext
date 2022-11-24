use termion::event::{Event, Key};

pub mod modes;
use modes::EditorMode;
use crate::utils::*;
use crate::renderer::Renderer;
use crate::renderer::types::*;
use crate::renderer::buffer::Buffer;
use crate::renderer::cursor::*;

pub struct CommandHandler {
    curr_mode: EditorMode,
}

impl CommandHandler {
    pub fn default() -> Self {
        Self {
            curr_mode: EditorMode::Normal
        }
    }


    pub fn handle(&mut self,
        event: Option<Event>, 
        renderer: &mut Renderer, 
        buf: &mut Buffer,
        cursor: &mut Cursor
    ) {
        match event {
            None => return,
            Some(e) => {
                match e {
                    Event::Key(_c) => self.handle_key(_c, renderer, buf, cursor),
                    _ => println!("Unsupported event {:?}", e),
                }
            }
        }
    }

    fn handle_key(&mut self,
        key: Key,
        renderer: &mut Renderer,
        buf: &mut Buffer,
        cursor: &mut Cursor
    ) {
        match self.curr_mode {
            EditorMode::Insert => {
                match key {
                    Key::Char(_c) => { 
                        let p = cursor.pos();
                        buf.insert(_c, p.y - 1, p.x - 1) ;
                        cursor.move_cursor(Direction::Right);
                    },
                    Key::Backspace => { 
                        let p = cursor.pos();
                        buf.backspace(p.y - 1, p.x - 1) ;
                        cursor.move_cursor(Direction::Left);
                    },
                    Key::Esc => { 
                        self.curr_mode = EditorMode::Normal;
                        print!("{}", termion::cursor::SteadyBlock);
                    },
                    _ => return,
                }
            },

            EditorMode::Normal => {
                match key {
                    Key::Char('i') => {
                        self.curr_mode = EditorMode::Insert;
                        print!("{}", termion::cursor::BlinkingBar);
                    },
                    Key::Esc => self.curr_mode = EditorMode::Normal,
                    Key::Char('q') => die(None),
                    Key::Char('h') => cursor.move_cursor(Direction::Left),
                    Key::Char('j') => cursor.move_cursor(Direction::Down),
                    Key::Char('k') => cursor.move_cursor(Direction::Up),
                    Key::Char('l') => cursor.move_cursor(Direction::Right),
                    _ => return,
                }
            },

            EditorMode::Command => {
            },
        }
    }
}
