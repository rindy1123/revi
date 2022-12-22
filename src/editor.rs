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
    pub fn run(&self) -> Result<(), std::io::Error> {
        loop {
            self.refresh_screen()?;
            if self.process_key_press()? {
                Terminal::clear_screen();
                return Ok(());
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor();
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
        Terminal::move_cursor(&Position::default());
        Terminal::show_cursor();
        Terminal::flush()
    }

    fn process_key_press(&self) -> Result<bool, std::io::Error> {
        let key = Terminal::read_key()?;
        match key {
            Key::Ctrl('q') => return Ok(true),
            _ => (),
        }
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
