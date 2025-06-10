pub struct TerminalState {
    pub current_input: String,
    pub history: Vec<String>,
    pub cursor_position: usize,
}

impl TerminalState {
    pub fn new() -> Self {
        Self {
            current_input: String::new(),
            history: vec![
                "Welcome to Warp Terminal!".to_string(),
                "Type 'help' for available commands or 'ai <query>' for AI assistance.".to_string(),
            ],
            cursor_position: 0,
        }
    }

    pub fn add_char(&mut self, c: char) {
        self.current_input.insert(self.cursor_position, c);
        self.cursor_position += 1;
    }

    pub fn backspace(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.current_input.remove(self.cursor_position);
        }
    }

    pub fn clear_input(&mut self) {
        self.current_input.clear();
        self.cursor_position = 0;
    }

    pub fn add_history_entry(&mut self, entry: String) {
        self.history.push(entry);
        // Keep only last 1000 entries
        if self.history.len() > 1000 {
            self.history.remove(0);
        }
    }

    pub fn add_error(&mut self, error: String) {
        self.history.push(format!("❌ {}", error));
    }
}
