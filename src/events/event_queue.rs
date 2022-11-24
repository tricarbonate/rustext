


pub struct EventQueue<T> {
    queue: Vec<T>,
}

impl<T> EventQueue<T> {
    pub fn default() -> Self {
        EventQueue { queue: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.queue.push(item)
    }

    pub fn pop(&mut self) -> T {
        self.queue.remove(0)
    }

    pub fn length(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    pub fn first(&self) -> Option<&T> {
        self.queue.first()
    }
}
