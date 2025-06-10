use super::{KeySet, KeyBinding};

pub fn default_keyset() -> KeySet {
    KeySet {
        name: "Default".to_string(),
        description: Some("Default Warp keybindings".to_string()),
        author: Some("Warp Team".to_string()),
        version: "1.0.0".to_string(),
        bindings: vec![
            KeyBinding {
                key: "c".to_string(),
                modifiers: vec!["ctrl".to_string()],
                action: "copy".to_string(),
                args: None,
                when: Some("selection".to_string()),
            },
            KeyBinding {
                key: "v".to_string(),
                modifiers: vec!["ctrl".to_string()],
                action: "paste".to_string(),
                args: None,
                when: None,
            },
            KeyBinding {
                key: "t".to_string(),
                modifiers: vec!["ctrl".to_string()],
                action: "new_tab".to_string(),
                args: None,
                when: None,
            },
            KeyBinding {
                key: "w".to_string(),
                modifiers: vec!["ctrl".to_string()],
                action: "close_tab".to_string(),
                args: None,
                when: None,
            },
            KeyBinding {
                key: "n".to_string(),
                modifiers: vec!["ctrl".to_string()],
                action: "new_window".to_string(),
                args: None,
                when: None,
            },
            KeyBinding {
                key: "f".to_string(),
                modifiers: vec!["ctrl".to_string()],
                action: "search".to_string(),
                args: None,
                when: None,
            },
        ],
    }
}

pub fn emacs_keyset() -> KeySet {
    KeySet {
        name: "Emacs".to_string(),
        description: Some("Emacs-style keybindings".to_string()),
        author: Some("Warp Team".to_string()),
        version: "1.0.0".to_string(),
        bindings: vec![
            KeyBinding {
                key: "a".to_string(),
                modifiers: vec!["ctrl".to_string()],
                action: "move_to_line_start".to_string(),
                args: None,
                when: None,
            },
            KeyBinding {
                key: "e".to_string(),
                modifiers: vec!["ctrl".to_string()],
                action: "move_to_line_end".to_string(),
                args: None,
                when: None,
            },
            KeyBinding {
                key: "k".to_string(),
                modifiers: vec!["ctrl".to_string()],
                action: "kill_line".to_string(),
                args: None,
                when: None,
            },
            KeyBinding {
                key: "u".to_string(),
                modifiers: vec!["ctrl".to_string()],
                action: "kill_line_backward".to_string(),
                args: None,
                when: None,
            },
            KeyBinding {
                key: "w".to_string(),
                modifiers: vec!["ctrl".to_string()],
                action: "kill_word_backward".to_string(),
                args: None,
                when: None,
            },
        ],
    }
}

pub fn vim_keyset() -> KeySet {
    KeySet {
        name: "Vim".to_string(),
        description: Some("Vim-style keybindings".to_string()),
        author: Some("Warp Team".to_string()),
        version: "1.0.0".to_string(),
        bindings: vec![
            KeyBinding {
                key: "h".to_string(),
                modifiers: vec![],
                action: "move_left".to_string(),
                args: None,
                when: Some("normal_mode".to_string()),
            },
            KeyBinding {
                key: "j".to_string(),
                modifiers: vec![],
                action: "move_down".to_string(),
                args: None,
                when: Some("normal_mode".to_string()),
            },
            KeyBinding {
                key: "k".to_string(),
                modifiers: vec![],
                action: "move_up".to_string(),
                args: None,
                when: Some("normal_mode".to_string()),
            },
            KeyBinding {
                key: "l".to_string(),
                modifiers: vec![],
                action: "move_right".to_string(),
                args: None,
                when: Some("normal_mode".to_string()),
            },
        ],
    }
}
