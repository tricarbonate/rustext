use std::io::{self, stdout, Read, Write};

use rim::renderer::Renderer;
use rim::events::EventListener;
use rim::commands::CommandHandler;
use rim::renderer::buffer::Buffer;
use rim::renderer::cursor::Cursor;
use rim::renderer::types::*;

fn main() {

    let mut eventListener = EventListener::default();
    let mut commandHandler = CommandHandler::default();
    let mut renderer = Renderer::default().expect("Error");
    let mut cursor = Cursor::default();

    let mut buffer = Buffer::from_example();

    loop { 
        renderer.refresh_screen(&buffer);
        update_cursor_pos(cursor.pos());
        
        // Listen for key press and other terminal events
        eventListener.listen();


        // handle one event from the events queue at a time
        commandHandler.handle(
            eventListener.dequeue(),
            &mut renderer,
            &mut buffer,
            &mut cursor
        );
    }
}

fn update_cursor_pos(pos: &Position) {
    print!("{}", termion::cursor::Goto(pos.x as u16, pos.y as u16));
    io::stdout().flush();
}
