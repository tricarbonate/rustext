use termion::event::{Event, Key};

pub mod modes;
// use modes::EditorMode;
use crate::utils::*;
use crate::renderer::Renderer;
use crate::renderer::types::*;
use crate::buffer::buffer::Buffer;
use crate::buffer::buffers_handler::BuffersHandler;
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
        b_handler: &mut BuffersHandler,
        cursor: &mut Cursor,
        status: &mut Status
    ) {
        match event {
            None => return,
            Some(e) => {
                match e {
                    Event::Key(_c) => self.handle_key(_c, renderer,
                        b_handler, cursor, status),
                    _ => println!("Unsupported event {:?}", e),
                }
            }
        }
    }

    fn handle_key(&mut self,
        key: Key,
        renderer: &mut Renderer,
        b_handler: &mut BuffersHandler,
        cursor: &mut Cursor,
        status: &mut Status
    ) {
        let mut buf = b_handler.get_current_buffer();
        match status.mode() {
            EditorMode::Insert => {
                match key {
                    Key::Char(_c) => { 
                        if _c == '\n' {
                            buf.insert_row(cursor.pos().y);
                            cursor.move_cursor(Direction::Down, buf);
                            return;
                        }
                        let p = cursor.pos();
                        buf.insert(_c, p.y - 1, p.x - 1) ;
                        cursor.insert_move();
                    },
                    Key::Backspace => { 
                        let p = cursor.pos();
                        buf.backspace(p.y - 1, p.x - 1) ;
                        cursor.backspace_move(buf);
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
                    Key::Char('a') => {
                        status.set_mode(EditorMode::Insert);
                        // cursor to the right without clipping
                        cursor.insert_move();
                        print!("{}", termion::cursor::BlinkingBar);
                    }
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
                                b_handler,
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

    fn execute_command_input(
        &mut self,
        cmd: String,
        // buf: &mut Buffer,
        b_handler: &mut BuffersHandler,
        status: &mut Status
    ) {
        let buf = b_handler.get_current_buffer();
        let split = &cmd[..].split_whitespace().collect::<Vec<&str>>();

        if split.len() <= 0 {
            return
        }

        match split[0] {
            "w" => {
                if let Some(arg) = split.get(1) {
                    buf.save(Some(String::from(*arg)));
                } else {
                    buf.save(None);
                }
                status.set_message(String::from("Saved"));
            },
            "o" => {
                if let Some(arg) = split.get(1) {
                    b_handler.load_from_file(String::from(*arg));
                } else {
                    println!("Argument required");
                }
            }
            "q" => {
                for b in b_handler.buffers() {
                    if !b.saved {
                        status.set_message(String::from(b.name.clone() + " save pls"));
                        return;
                    }
                }
                die(None)
            },
            _ => {}
        }
    }
}
