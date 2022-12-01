


pub struct CommandHistory {
    command_line_hist: Vec<String>,
    pointer: usize
}

impl CommandHistory {
    pub fn default() -> Self {
        Self {
            command_line_hist: Vec::new(),
            pointer: 0
        }
    }

    pub fn push_command_line_hist(&mut self, cmd: String) {
        self.command_line_hist.push(cmd);
        self.pointer = self.size()
    }

    pub fn go_down_history(&mut self) -> String {
        if self.pointer > 0 { self.pointer -= 1 }
        if self.size() == 0 { return String::from("") }
        self.command_line_hist[self.pointer].clone()
    }

    pub fn go_up_history(&mut self) -> String {
        if self.pointer < self.size() { self.pointer += 1 }
        if self.pointer == self.size() { return String::from("") }
        self.command_line_hist[self.pointer].clone()
    }

    pub fn size(&self) -> usize {
        self.command_line_hist.len()
    }
}
