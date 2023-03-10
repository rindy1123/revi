use std::{cmp, env};

use termion::{color, event::Key};

use crate::{document::Document, terminal::Terminal};

pub struct Editor {
    terminal: Terminal,
    cursor_position: Position,
    offset: Position,
    document: Document,
    mode: Mode,
}

#[derive(Default)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

enum Mode {
    Normal,
    Insert,
}

const STATUS_FG_COLOR: color::Rgb = color::Rgb(255, 255, 255);
const STATUS_BG_COLOR: color::Rgb = color::Rgb(20, 130, 241);

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
            offset: Position::default(),
            mode: Mode::Normal,
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
        let height = self.terminal.size().height as usize;
        let width = self.terminal.size().width as usize;
        for line_num in 0..height {
            Terminal::clear_current_line();
            if let Some(row) = self.document.row(line_num + self.offset.y) {
                let start = self.offset.x;
                let end = self.offset.x.saturating_add(width);
                println!("{}\r", row.render(start, end));
            } else {
                println!("~\r");
            }
        }
        self.draw_status_bar();
        self.draw_message_bar();
        let cursor_position = Position {
            x: self.cursor_position.x.saturating_sub(self.offset.x),
            y: self.cursor_position.y.saturating_sub(self.offset.y),
        };
        Terminal::move_cursor(&cursor_position);
        Terminal::show_cursor();
        Terminal::flush()
    }

    fn process_key_press(&mut self) -> Result<bool, std::io::Error> {
        let key = Terminal::read_key()?;
        let quit = match self.mode {
            Mode::Normal => self.process_normal_mode(key),
            Mode::Insert => self.process_insert_mode(key),
        };
        Ok(quit)
    }

    fn process_normal_mode(&mut self, key: Key) -> bool {
        match key {
            Key::Ctrl('q') => return true,
            Key::Char('l' | 'h' | 'j' | 'k') => {
                self.move_cursor(key);
            }
            Key::Char('i') => {
                self.mode = Mode::Insert;
            }
            _ => (),
        }
        false
    }

    fn move_cursor(&mut self, key: Key) {
        let Position { mut x, mut y } = self.cursor_position;
        let height = self.terminal.size().height as usize;
        let width = self.terminal.size().width as usize;
        let row_len = match self.document.row(y) {
            Some(row) => row.len(),
            None => 0,
        };
        match key {
            Key::Char('l') if row_len > x + 1 => {
                x = x.saturating_add(1);
                if x >= width + self.offset.x {
                    self.offset.x += 1;
                }
            }
            Key::Char('h') => {
                x = x.saturating_sub(1);
                if self.offset.x > x {
                    self.offset.x -= 1;
                }
            }
            Key::Char('j') if self.document.len() > y + 1 => {
                y = y.saturating_add(1);
                if y >= height + self.offset.y {
                    self.offset.y += 1;
                }
            }
            Key::Char('k') => {
                y = y.saturating_sub(1);
                if self.offset.y > y {
                    self.offset.y -= 1;
                }
            }
            Key::Char('i') => {
                self.mode = Mode::Insert;
            }
            _ => (),
        }
        // If the row the cursor is on is shorter than the previous one,
        // the cursor moves to the last character of the current row.
        let x = match self.document.row(y) {
            Some(row) => cmp::min(x, row.len().saturating_sub(1)),
            None => 0,
        };
        if self.offset.x > x {
            self.offset.x = x;
        }
        self.cursor_position = Position { x, y };
    }

    fn process_insert_mode(&mut self, key: Key) -> bool {
        match key {
            Key::Char(c) => {
                self.document.insert(&self.cursor_position, c);
                self.move_cursor(Key::Char('l'));
            }
            Key::Esc => {
                self.mode = Mode::Normal;
            }
            _ => (),
        }
        false
    }

    fn draw_status_bar(&self) {
        Terminal::clear_current_line();

        let width = self.terminal.size().width as usize;
        Terminal::set_bg_color(STATUS_BG_COLOR);
        Terminal::set_fg_color(STATUS_FG_COLOR);
        let mode = match self.mode {
            Mode::Normal => String::from("[NORMAL]"),
            Mode::Insert => String::from("[INSERT]"),
        };
        let status_bar = format!("{}{}", mode, " ".repeat(width - mode.len()));
        println!("{status_bar}\r");
        Terminal::reset_fg_color();
        Terminal::reset_bg_color();
    }

    fn draw_message_bar(&self) {
        Terminal::clear_current_line();
        print!("\r");
    }
}
