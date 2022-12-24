use std::{env, io};

use termion::{event::Key, input::TermRead};

use crate::{
    document::Document,
    terminal::{Size, Terminal},
};

pub struct Editor {
    terminal: Terminal,
    cursor_position: Position,
    document: Document,
}

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Default for Editor {
    fn default() -> Self {
        let args: Vec<String> = env::args().collect();
        let document = match args.get(1) {
            Some(file_name) => Document::open(file_name).unwrap_or_else(|_| Document::default()),
            None => Document::default(),
        };
        Self {
            terminal: Terminal::default(),
            cursor_position: Position::default(),
            document,
        }
    }
}

impl Editor {
    pub fn run(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh_screen()?;
            if self.process_key_press()? {
                Terminal::clear_screen();
                Terminal::move_cursor(&Position::default());
                return Ok(());
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor();
        Terminal::move_cursor(&Position::default());
        let height = self.terminal.size().height;
        for line_num in 0..height {
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(line_num as usize) {
                println!("{}\r", row.string);
            } else {
                println!("~\r");
            }
        }
        self.draw_status_bar();
        self.draw_message_bar();
        Terminal::move_cursor(&self.cursor_position);
        Terminal::show_cursor();
        Terminal::flush()
    }

    fn process_key_press(&mut self) -> Result<bool, std::io::Error> {
        let key = Terminal::read_key()?;
        let Position { mut x, mut y } = self.cursor_position;
        let height = self.terminal.size().height as usize;
        match key {
            Key::Ctrl('q') => return Ok(true),
            Key::Char('l') => {
                x = x.saturating_add(1);
            }
            Key::Char('h') => {
                x = x.saturating_sub(1);
            }
            Key::Char('j') if height - 1 > y => {
                y = y.saturating_add(1);
            }
            Key::Char('k') => {
                y = y.saturating_sub(1);
            }
            _ => (),
        }
        self.cursor_position = Position { x, y };
        Terminal::flush()?;
        Ok(false)
    }

    fn draw_status_bar(&self) {
        Terminal::clear_current_line();
        println!("~\r");
    }

    fn draw_message_bar(&self) {
        Terminal::clear_current_line();
        print!("~\r");
    }
}
