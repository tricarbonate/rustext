use crate::buffer::buffers_handler::BuffersHandler;

pub enum EditorMode {
    Normal,
    Insert,
    Command
}

pub struct Status {
    // actual text written on the status bar
    message: String,
    mode: EditorMode,
    // name of currently opened buffer
    buffer_name: String,
    command_line_input: String,
}


impl Status {
    pub fn default() -> Self {
        Self {
            message: String::new(), 
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
            + "  @"
            + &self.buffer_name[..]
            + "  Message: "
            + &self.message[..]
            + "  => "
        // self.text.clone()
    }

    pub fn init(&mut self) {
        self.set_mode(EditorMode::Normal);
        print!("{}", termion::cursor::SteadyBlock);
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

    pub fn update(&mut self, bufs: &BuffersHandler) {
        self.buffer_name = bufs.get_name()
    }

    pub fn set_mode(&mut self, mode: EditorMode) {
        self.mode = mode;
    }

    pub fn set_message(&mut self, mes: String) {
        self.message = mes;
    }

    pub fn mode(&self) -> &EditorMode {
        &self.mode
    }
}
