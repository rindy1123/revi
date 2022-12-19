#![warn(clippy::all, clippy::pedantic)]

use editor::Editor;

mod editor;
mod terminal;

fn main() {
    Editor::default().run();
}
