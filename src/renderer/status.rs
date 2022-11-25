use std::time::Duration;
use std::time::Instant;
use crate::renderer::buffer::Buffer;

pub enum EditorMode {
    Normal,
    Insert,
    Command
}

pub struct Status {
    // actual text written on the status bar
    text: String,
    time: Instant,
    mode: EditorMode,
    buffer_name: String
}


impl Status {
    pub fn from(message: String) -> Self {
        Self {
            time: Instant::now(),
            text: message,
            mode: EditorMode::Normal,
            buffer_name: String::from("unnamed")
        }
    }

    /*
     * Returns formatted status line, ready to be drawn
     */
    pub fn formatted(&self) -> String {
        let mode_str = match self.mode {
            EditorMode::Normal => "[Normal]",
            EditorMode::Insert => "[Insert]",
            EditorMode::Command => "[Command]",
        };
        self.buffer_name.clone()
        // String::from(mode_str).push_str(&self.buffer_name) 
        // self.text.clone()
    }

    pub fn update(&mut self, buf: &Buffer) {
        self.buffer_name = buf.name()
    }

    pub fn set_mode(&mut self, mode: EditorMode) {
        self.mode = mode;
    }

    pub fn set_message(&mut self, mes: String) {
        self.text = mes;
    }

    pub fn mode(&self) -> &EditorMode {
        &self.mode
    }
}
