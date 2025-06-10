use super::{WarpTheme, ThemeColors, AnsiColors, ThemeUI, ThemeTerminal};

pub fn dark_theme() -> WarpTheme {
    WarpTheme {
        name: "Standard Dark".to_string(),
        author: Some("Warp Team".to_string()),
        version: "1.0.0".to_string(),
        description: Some("The default dark theme for Warp".to_string()),
        colors: ThemeColors {
            background: "#1e1e2e".to_string(),
            foreground: "#cdd6f4".to_string(),
            cursor: "#f5e0dc".to_string(),
            selection_background: "#585b70".to_string(),
            selection_foreground: "#cdd6f4".to_string(),
            ansi: AnsiColors {
                black: "#45475a".to_string(),
                red: "#f38ba8".to_string(),
                green: "#a6e3a1".to_string(),
                yellow: "#f9e2af".to_string(),
                blue: "#89b4fa".to_string(),
                magenta: "#f5c2e7".to_string(),
                cyan: "#94e2d5".to_string(),
                white: "#bac2de".to_string(),
            },
            bright: AnsiColors {
                black: "#585b70".to_string(),
                red: "#f38ba8".to_string(),
                green: "#a6e3a1".to_string(),
                yellow: "#f9e2af".to_string(),
                blue: "#89b4fa".to_string(),
                magenta: "#f5c2e7".to_string(),
                cyan: "#94e2d5".to_string(),
                white: "#a6adc8".to_string(),
            },
        },
        ui: ThemeUI {
            accent: "#89b4fa".to_string(),
            border: "#6c7086".to_string(),
            tab_active: "#313244".to_string(),
            tab_inactive: "#181825".to_string(),
            status_bar: "#313244".to_string(),
            menu_background: "#313244".to_string(),
            menu_foreground: "#cdd6f4".to_string(),
        },
        terminal: ThemeTerminal {
            bright_bold: true,
            cursor_style: "block".to_string(),
            cursor_blink: true,
        },
    }
}

pub fn light_theme() -> WarpTheme {
    WarpTheme {
        name: "Standard Light".to_string(),
        author: Some("Warp Team".to_string()),
        version: "1.0.0".to_string(),
        description: Some("The default light theme for Warp".to_string()),
        colors: ThemeColors {
            background: "#eff1f5".to_string(),
            foreground: "#4c4f69".to_string(),
            cursor: "#dc8a78".to_string(),
            selection_background: "#acb0be".to_string(),
            selection_foreground: "#4c4f69".to_string(),
            ansi: AnsiColors {
                black: "#5c5f77".to_string(),
                red: "#d20f39".to_string(),
                green: "#40a02b".to_string(),
                yellow: "#df8e1d".to_string(),
                blue: "#1e66f5".to_string(),
                magenta: "#ea76cb".to_string(),
                cyan: "#179299".to_string(),
                white: "#acb0be".to_string(),
            },
            bright: AnsiColors {
                black: "#6c6f85".to_string(),
                red: "#d20f39".to_string(),
                green: "#40a02b".to_string(),
                yellow: "#df8e1d".to_string(),
                blue: "#1e66f5".to_string(),
                magenta: "#ea76cb".to_string(),
                cyan: "#179299".to_string(),
                white: "#bcc0cc".to_string(),
            },
        },
        ui: ThemeUI {
            accent: "#1e66f5".to_string(),
            border: "#9ca0b0".to_string(),
            tab_active: "#dce0e8".to_string(),
            tab_inactive: "#e6e9ef".to_string(),
            status_bar: "#dce0e8".to_string(),
            menu_background: "#dce0e8".to_string(),
            menu_foreground: "#4c4f69".to_string(),
        },
        terminal: ThemeTerminal {
            bright_bold: true,
            cursor_style: "block".to_string(),
            cursor_blink: true,
        },
    }
}
