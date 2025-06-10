use std::collections::VecDeque;

/// Represents the current state of the terminal emulator
#[derive(Debug)]
pub struct TerminalState {
    /// Current user input buffer
    pub current_input: String,
    /// Command history (most recent last)
    pub history: VecDeque<String>,
    /// Current cursor position in bytes
    pub cursor_position: usize,
}

impl TerminalState {
    /// Creates a new TerminalState with default welcome messages
    pub fn new() -> Self {
        Self::with_welcome_messages(vec![
            "Welcome to Warp Terminal!".to_string(),
            "Type 'help' for available commands or 'ai <query>' for AI assistance.".to_string(),
        ])
    }

    /// Creates a new TerminalState with custom welcome messages
    pub fn with_welcome_messages(messages: Vec<String>) -> Self {
        Self {
            current_input: String::new(),
            history: VecDeque::from(messages),
            cursor_position: 0,
        }
    }

    /// Adds a character at the current cursor position
    pub fn add_char(&mut self, c: char) {
        let byte_idx = self.get_byte_index();
        self.current_input.insert(byte_idx, c);
        self.cursor_position += c.len_utf8();
    }

    /// Deletes the character before the cursor
    pub fn backspace(&mut self) {
        if self.cursor_position > 0 {
            let byte_idx = self.get_byte_index();
            let prev_char = self.current_input[..byte_idx]
                .chars()
                .last()
                .unwrap();
            self.current_input.remove(byte_idx - prev_char.len_utf8());
            self.cursor_position -= prev_char.len_utf8();
        }
    }

    /// Gets the current byte index based on cursor position
    fn get_byte_index(&self) -> usize {
        self.current_input
            .char_indices()
            .take_while(|(i, _)| *i <= self.cursor_position)
            .map(|(i, _)| i)
            .last()
            .unwrap_or(0)
    }

    /// Clears the current input buffer
    pub fn clear_input(&mut self) {
        self.current_input.clear();
        self.cursor_position = 0;
    }

    /// Adds a new entry to the history
    pub fn add_history_entry(&mut self, entry: String) {
        self.history.push_back(entry);
        if self.history.len() > 1000 {
            self.history.pop_front();
        }
    }

    pub fn add_error(&mut self, error: String) {
        self.history.push_back(format!("❌ {}", error));
    }
}
