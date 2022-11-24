use std::io::{self, stdout, Read, Write};

use rim::renderer::Renderer;
use rim::events::EventListener;
use rim::commands::CommandHandler;
use rim::renderer::buffer::Buffer;

fn main() {

    let mut eventListener = EventListener::default();
    let mut commandHandler = CommandHandler::default();
    let mut renderer = Renderer::default().expect("Error");

    let mut buffer = Buffer::from_example();

    loop { 
        renderer.refresh_screen(&buffer);
        eventListener.listen();
        commandHandler.handle(eventListener.dequeue(), &mut renderer, &mut buffer);

        // renderer.draw_buffer(&buffer);
        // io::stdout().flush();
    }
}
