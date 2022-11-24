use termion::event::Event;
use termion::input::TermRead;
use std::io;

pub mod event_queue;
use event_queue::EventQueue;

pub struct EventListener {
    events: EventQueue<Event>,
}

impl EventListener {
    pub fn default() -> Self {
        Self {
            events: EventQueue::default()
        }
    }

    /*
     * Registers every events in the event queue
     */
    pub fn listen(&mut self) {
        for event in io::stdin().events() {
            let _ = match event {
                Ok(e) => {
                    self.events.push(e);
                    return;
                },
                Err(err) => err,
            };
        } 
    }

    /*
     * Returns the next event to be scanned, and consumes it
     */
    pub fn dequeue(&mut self) -> Option<Event> {
        match self.events.is_empty() {
            true => None, 
            false => Some(self.events.pop())
        }
    }
}
