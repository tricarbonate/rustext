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
    buffer_name: String,
    command_line_input: String,
}


impl Status {
    pub fn from(message: String) -> Self {
        Self {
            time: Instant::now(),
            text: message,
            mode: EditorMode::Normal,
            buffer_name: String::from("unnamed"),
            command_line_input: String::from("")
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
        // self.buffer_name.clone()
        String::from(mode_str)
        // self.text.clone()
    }

    pub fn set_command_line_input(&mut self, c: String) {
        self.command_line_input = c;
    }

    pub fn command_line_input(&self) -> String {
        self.command_line_input.clone()
    }

    pub fn reset_command_line_input(&mut self) { 
        self.command_line_input = String::from("");
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
