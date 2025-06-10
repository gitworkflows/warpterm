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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state() {
        let state = TerminalState::new();
        assert!(state.current_input.is_empty());
        assert_eq!(state.history.len(), 2);
        assert_eq!(state.cursor_position, 0);
    }

    #[test]
    fn test_add_char_ascii() {
        let mut state = TerminalState::new();
        state.add_char('a');
        assert_eq!(state.current_input, "a");
        assert_eq!(state.cursor_position, 1);
    }

    #[test]
    fn test_add_char_utf8() {
        let mut state = TerminalState::new();
        state.add_char('á'); // 2-byte char
        assert_eq!(state.current_input, "á");
        assert_eq!(state.cursor_position, 2);
    }

    #[test]
    fn test_backspace() {
        let mut state = TerminalState::new();
        state.add_char('a');
        state.add_char('b');
        state.backspace();
        assert_eq!(state.current_input, "a");
        assert_eq!(state.cursor_position, 1);
    }

    #[test]
    fn test_history_trimming() {
        let mut state = TerminalState::new();
        for i in 0..1005 {
            state.add_history_entry(format!("Command {}", i));
        }
        assert_eq!(state.history.len(), 1000);
        assert_eq!(state.history.front().unwrap(), "Command 5");
    }
}
