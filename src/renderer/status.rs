use std::time::Duration;
use std::time::Instant;

pub enum EditorMode {
    Normal,
    Insert,
    Command
}

pub struct Status {
    // actual text written on the status bar
    text: String,
    time: Instant,
    mode: EditorMode 
}


impl Status {
    pub fn from(message: String) -> Self {
        Self {
            time: Instant::now(),
            text: message,
            mode: EditorMode::Normal
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
        String::from(mode_str)
        // self.text.clone()
    }

    pub fn set_mode(&mut self, mode: EditorMode) {
        self.mode = mode;
    }

    pub fn mode(&self) -> &EditorMode {
        &self.mode
    }
}
