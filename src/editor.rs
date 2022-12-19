use std::io::{self, Write};

use termion::{event::Key, input::TermRead, raw::IntoRawMode};

#[derive(Default)]
pub struct Editor {}

impl Editor {
    pub fn run(&self) -> Result<(), std::io::Error> {
        let _stdout = io::stdout().into_raw_mode()?;
        for key in io::stdin().lock().keys() {
            let key = key?;
            match key {
                Key::Ctrl('q') => return Ok(()),
                Key::Char(c) => print!("{}", c),
                _ => (),
            }
            io::stdout().flush()?;
        }
        Ok(())
    }
}
