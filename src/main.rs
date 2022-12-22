#![warn(clippy::all, clippy::pedantic)]

use editor::Editor;

mod document;
mod editor;
mod row;
mod terminal;

fn main() {
    Editor::default().run();
}
