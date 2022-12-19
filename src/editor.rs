use std::io;

use termion::{event::Key, input::TermRead};

use crate::terminal::{Size, Terminal};

#[derive(Default)]
pub struct Editor {
    terminal: Terminal,
    cursor_position: Position,
}

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
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
        let Size { height, .. } = self.terminal.size();
        for _ in 0..*height {
            Terminal::clear_current_line();
            println!("~\r");
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
        println!("~\r");
    }
}
