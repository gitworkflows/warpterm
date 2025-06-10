use serde::{Deserialize, Serialize};
use crossterm::style::Color;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub background: Color,
    pub foreground: Color,
    pub cursor: Color,
    pub selection: Color,
    pub accent: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            name: "Dark".to_string(),
            background: Color::Black,
            foreground: Color::White,
            cursor: Color::Green,
            selection: Color::Blue,
            accent: Color::Cyan,
        }
    }
}
