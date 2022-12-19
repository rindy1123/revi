use std::io::{self, Write};
use termion::{
    event::Key,
    input::TermRead,
    raw::{IntoRawMode, RawTerminal},
};

use crate::editor::Position;

pub struct Terminal {
    size: Size,
    _stdout: RawTerminal<std::io::Stdout>,
}

pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Default for Terminal {
    fn default() -> Self {
        let (width, height) = termion::terminal_size().expect("Failed to get terminal size.");
        let height = height.saturating_sub(2);
        let size = Size { width, height };
        let stdout = io::stdout()
            .into_raw_mode()
            .expect("Terminal failed to turn into raw mode.");
        Self {
            size,
            _stdout: stdout,
        }
    }
}

impl Terminal {
    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn flush() -> Result<(), std::io::Error> {
        io::stdout().flush()
    }

    pub fn show_cursor() {
        print!("{}", termion::cursor::Show);
    }

    pub fn hide_cursor() {
        print!("{}", termion::cursor::Hide);
    }

    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }

    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    pub fn read_key() -> Result<Key, std::io::Error> {
        loop {
            if let Some(key) = io::stdin().lock().keys().next() {
                return key;
            }
        }
    }

    pub fn move_cursor(position: &Position) {
        let Position { mut x, mut y } = position;
        x = x.saturating_add(1);
        y = y.saturating_add(1);
        print!("{}", termion::cursor::Goto(x as u16, y as u16));
    }
}
