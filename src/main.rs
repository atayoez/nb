use crate::cli::Cli;
use crate::editor::Editor;
use crate::flags::Flags;
use clap::Parser;

mod cli;
mod editor;
mod flags;

fn main() {
    let cli = Cli::parse();
    let flags = Flags::new(cli);
    let editor = Editor::new(flags);
    editor.display();
}
