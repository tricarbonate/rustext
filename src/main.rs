use rim::renderer::renderer::Renderer;
use rim::events::EventListener;
use rim::commands::commands_handler::CommandHandler;
// use rim::settings::*;
use rim::renderer::cursor::Cursor;
use rim::renderer::status::Status;
use rim::buffer::buffers_handler::BuffersHandler;

fn main() {

    let mut event_listener = EventListener::default();
    let mut command_handler = CommandHandler::default();
    let mut renderer = Renderer::default().expect("Error");
    let mut status = Status::default();
    let mut cursor = Cursor::default();
    // let settings = Settings::default();

    let mut b_handler = BuffersHandler::default();

    status.init();

    loop {

        status.update(&b_handler);
        renderer.refresh_screen(&mut b_handler, &status);
        Cursor::update_real_cursor_pos(cursor.pos());
        
        // Listen for key press and other terminal events
        event_listener.listen();


        // handle one event from the events queue at a time
        command_handler.handle(
            event_listener.dequeue(),
            &mut b_handler,
            &mut cursor,
            &mut status
        );
    }
}
