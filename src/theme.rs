use crossterm::style::Color;

pub struct Theme {
    pub accent_color: Color,
    pub prompt_color: Color,
    pub text_color: Color,
    pub history_color: Color,
    pub error_color: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            accent_color: Color::Cyan,
            prompt_color: Color::Green,
            text_color: Color::White,
            history_color: Color::DarkGrey,
            error_color: Color::Red,
        }
    }
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            accent_color: Color::Blue,
            prompt_color: Color::Yellow,
            text_color: Color::White,
            history_color: Color::DarkGrey,
            error_color: Color::Red,
        }
    }

    pub fn light() -> Self {
        Self {
            accent_color: Color::DarkBlue,
            prompt_color: Color::DarkGreen,
            text_color: Color::Black,
            history_color: Color::Grey,
            error_color: Color::DarkRed,
        }
    }
}
