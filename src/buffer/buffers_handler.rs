use std::collections::BTreeMap;
use crate::buffer::buffer::Buffer;

/*
 * Instantiate and manage all buffers
 */
pub struct BuffersHandler {
    buffers: Vec<Buffer>,
    current_buffer: String,
}

impl BuffersHandler {
    pub fn default() -> Self {
        let mut buffers = Vec::new();
        buffers.push(Buffer::default());
        Self {
            buffers,
            current_buffer: String::from("unnamed")
        }
    }

    pub fn load_from_file(&mut self, filename: String) {
        match self.get_buffer(filename.clone()) {
            Some(_) => {
                self.current_buffer = filename;
            },
            None => {
                self.create(filename);
            }
        }
    }

    pub fn create(&mut self, filename: String) {
        Buffer::from_file(filename);
    }

    pub fn get_buffer(&mut self, name: String) -> Option<&mut Buffer> {
        for b in self.buffers.iter_mut() {
            if b.name == name {
                return Some(b);
            }
        }

        None
    }

    pub fn get_current_buffer(&mut self) -> &mut Buffer {
        match self.get_buffer(self.current_buffer.clone()) {
            Some(b) => b,
            None => { panic!(); }
        }
    }

    // Returns current buffer name
    pub fn get_name(&self) -> String {
        self.current_buffer.clone()
    }

    pub fn buffers(&self) -> &Vec<Buffer> {
        &self.buffers
    }
}

