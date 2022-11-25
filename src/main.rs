use std::io::{self, stdout, Read, Write};

use rim::renderer::Renderer;
use rim::events::EventListener;
use rim::commands::CommandHandler;
use rim::buffer::buffer::Buffer;
use rim::renderer::cursor::Cursor;
use rim::renderer::types::*;
use rim::renderer::status::Status;

fn main() {

    let mut eventListener = EventListener::default();
    let mut commandHandler = CommandHandler::default();
    let mut renderer = Renderer::default().expect("Error");
    let mut status = Status::from(String::from("Hello"));
    let mut cursor = Cursor::default();

    let mut buffer = Buffer::from_file(String::from("example"));

    let mut buffer = match buffer {
        Err(e) => panic!("{}", e),
        Ok(buf) => buf,
    };

    loop { 
        // status.update
        status.update(&buffer);
        renderer.refresh_screen(&buffer, &status);
        update_cursor_pos(cursor.pos());
        
        // Listen for key press and other terminal events
        eventListener.listen();


        // handle one event from the events queue at a time
        commandHandler.handle(
            eventListener.dequeue(),
            &mut renderer,
            &mut buffer,
            &mut cursor,
            &mut status
        );
    }
}

fn update_cursor_pos(pos: &Position) {
    print!("{}", termion::cursor::Goto(pos.x as u16, pos.y as u16));
    io::stdout().flush();
}
