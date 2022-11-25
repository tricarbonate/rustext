use std::io::{self, stdout, Read, Write};

use rim::renderer::Renderer;
use rim::events::EventListener;
use rim::commands::CommandHandler;
use rim::buffer::buffer::Buffer;
use rim::renderer::cursor::Cursor;
use rim::renderer::types::*;
use rim::renderer::status::Status;
use rim::buffer::buffers_handler::BuffersHandler;

fn main() {

    let mut event_listener = EventListener::default();
    let mut command_handler = CommandHandler::default();
    let mut renderer = Renderer::default().expect("Error");
    let mut status = Status::from(String::from("Hello"));
    let mut cursor = Cursor::default();

    let mut b_handler = BuffersHandler::default();

    loop {

        status.update(&b_handler);
        renderer.refresh_screen(&mut b_handler, &status);
        update_cursor_pos(cursor.pos());
        
        // Listen for key press and other terminal events
        event_listener.listen();


        // handle one event from the events queue at a time
        command_handler.handle(
            event_listener.dequeue(),
            &mut renderer,
            &mut b_handler,
            &mut cursor,
            &mut status
        );
    }
}

fn update_cursor_pos(pos: &Position) {
    print!("{}", termion::cursor::Goto(pos.x as u16, pos.y as u16));
    io::stdout().flush();
}
