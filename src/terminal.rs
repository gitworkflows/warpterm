use crossterm::{
    cursor,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, ClearType},
    QueueableCommand,
};
use std::io::{self, Write};

use crate::error::WarpError;

pub struct Terminal {
    width: u16,
    height: u16,
    cursor_x: u16,
    cursor_y: u16,
    buffer: Vec<Vec<char>>,
}

impl Terminal {
    pub async fn new() -> Result<Self, WarpError> {
        let (width, height) = terminal::size()?;
        let buffer = vec![vec![' '; width as usize]; height as usize];

        Ok(Self {
            width,
            height,
            cursor_x: 0,
            cursor_y: 0,
            buffer,
        })
    }

    pub async fn resize(&mut self, width: u16, height: u16) -> Result<(), WarpError> {
        self.width = width;
        self.height = height;
        self.buffer = vec![vec![' '; width as usize]; height as usize];
        Ok(())
    }

    pub async fn clear(&mut self) -> Result<(), WarpError> {
        let mut stdout = io::stdout();
        stdout.queue(terminal::Clear(ClearType::All))?;
        stdout.queue(cursor::MoveTo(0, 0))?;
        stdout.flush()?;

        self.buffer = vec![vec![' '; self.width as usize]; self.height as usize];
        self.cursor_x = 0;
        self.cursor_y = 0;

        Ok(())
    }

    pub async fn write_at(&mut self, x: u16, y: u16, text: &str, color: Color) -> Result<(), WarpError> {
        let mut stdout = io::stdout();
        stdout.queue(cursor::MoveTo(x, y))?;
        stdout.queue(SetForegroundColor(color))?;
        stdout.queue(Print(text))?;
        stdout.queue(ResetColor)?;
        stdout.flush()?;

        // Update buffer
        if y < self.height && x < self.width {
            let chars: Vec<char> = text.chars().collect();
            for (i, &ch) in chars.iter().enumerate() {
                let pos_x = x + i as u16;
                if pos_x < self.width {
                    self.buffer[y as usize][pos_x as usize] = ch;
                }
            }
        }

        Ok(())
    }

    pub async fn move_cursor(&mut self, x: u16, y: u16) -> Result<(), WarpError> {
        let mut stdout = io::stdout();
        stdout.queue(cursor::MoveTo(x, y))?;
        stdout.flush()?;

        self.cursor_x = x;
        self.cursor_y = y;

        Ok(())
    }

    pub fn get_size(&self) -> (u16, u16) {
        (self.width, self.height)
    }

    pub fn get_cursor_position(&self) -> (u16, u16) {
        (self.cursor_x, self.cursor_y)
    }
}
