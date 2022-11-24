use termion::event::{Event, Key};

pub mod modes;
use modes::EditorMode;
use crate::utils::*;
use crate::renderer::{Renderer, Direction};
use crate::renderer::buffer::Buffer;

pub struct CommandHandler {
    curr_mode: EditorMode,
}

impl CommandHandler {
    pub fn default() -> Self {
        Self {
            curr_mode: EditorMode::Normal
        }
    }


    pub fn handle(&mut self, event: Option<Event>, renderer: &mut Renderer, buf: &mut Buffer) {
        match event {
            None => return,
            Some(e) => {
                match e {
                    Event::Key(_c) => self.handle_key(_c, renderer, buf),
                    _ => println!("Unsupported event {:?}", e),
                }
            }
        }
    }

    fn handle_key(&mut self, key: Key, renderer: &mut Renderer, buf: &mut Buffer) {
        match self.curr_mode {
            EditorMode::Insert => {
                match key {
                    Key::Char(_c) => { renderer.modify_buffer(_c, buf); },
                    Key::Backspace => { renderer.backspace_buffer(buf); },
                    Key::Esc => { 
                        self.curr_mode = EditorMode::Normal;
                        // renderer.move_cursor(Direction::Right);
                        print!("{}", termion::cursor::SteadyBlock);
                    },
                    _ => return,
                }
            },

            EditorMode::Normal => {
                match key {
                    Key::Char('i') => {
                        self.curr_mode = EditorMode::Insert;
                        // renderer.move_cursor(Direction::Left);
                        print!("{}", termion::cursor::BlinkingBar);
                    },
                    Key::Esc => self.curr_mode = EditorMode::Normal,
                    Key::Char('q') => die(None),
                    Key::Char('h') => renderer.move_cursor(Direction::Left),
                    Key::Char('j') => renderer.move_cursor(Direction::Down),
                    Key::Char('k') => renderer.move_cursor(Direction::Up),
                    Key::Char('l') => renderer.move_cursor(Direction::Right),
                    _ => return,
                }
            },

            EditorMode::Command => {
            },
        }
    }
}
