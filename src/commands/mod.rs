use termion::event::{Event, Key};

pub mod modes;
// use modes::EditorMode;
use crate::utils::*;
use crate::renderer::Renderer;
use crate::renderer::types::*;
use crate::renderer::buffer::Buffer;
use crate::renderer::cursor::*;
use crate::renderer::status::{Status, EditorMode};

pub struct CommandHandler {
}

impl CommandHandler {
    pub fn default() -> Self {
        Self {
        }
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
                    Key::Esc => status.set_mode(EditorMode::Normal), 
                    Key::Char('h') => cursor.move_cursor(Direction::Left, buf),
                    Key::Char('j') => cursor.move_cursor(Direction::Down, buf),
                    Key::Char('k') => cursor.move_cursor(Direction::Up, buf),
                    Key::Char('l') => cursor.move_cursor(Direction::Right, buf),
                    Key::Char(':') => {
                        status.set_mode(EditorMode::Command);
                    }
                    _ => return,
                }
            },

            EditorMode::Command => {
                match key {
                    // execute command
                    Key::Esc => {
                        status.set_mode(EditorMode::Normal);
                        status.reset_command_line_input();
                    }

                    // add character and draw
                    Key::Char(_c) => {
                        if _c == '\n' {
                            self.execute_command_input(
                                status.command_line_input(),
                                buf,
                                status
                            );
                            status.set_mode(EditorMode::Normal);
                            status.reset_command_line_input();
                            return
                        }
                        let mut tmp = [0u8, 4];
                        status.set_command_line_input(
                            status.command_line_input() + _c.encode_utf8(&mut tmp)
                        );
                    }
                    Key::Backspace => {
                        let mut s = status.command_line_input();
                        s.pop();
                        status.set_command_line_input(s);
                    }
                    _ => return
                }
            },
        }
    }

    fn execute_command_input(&mut self, cmd: String, buf: &Buffer, status: &mut Status) {
        match &cmd[..] {
            "w" => {
                buf.save();
                status.set_message(String::from("Saved"));
            },
            "q" => {
                die(None)
            }
            _ => {}
        }
    }
}
