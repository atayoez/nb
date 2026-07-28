use crate::cli::Cli;
use crate::term::{clear_screen, enable_raw_mode, read_key, Key};
use clap::Parser;
use std::fs::File;
use std::io::Read;

mod cli;
mod term;

fn main() {
    let cli = Cli::parse();
    let mut create_new_file = false;
    let mut is_edited: bool = false;
    let mut path = String::new();
    let mut buffer = String::new();

    if let Some(p) = &cli.path {
        path = p.to_str().unwrap().to_string();
    } else {
        create_new_file = true;
    }

    if !create_new_file {
        let file = File::open(path);
        if let Ok(mut content) = file {
            content.read_to_string(&mut buffer).unwrap();
        } else {
            println!("Could not read file");
        }
    }

    // Display
    display(&mut buffer, is_edited, create_new_file);
}

fn display(buffer: &mut String, is_edited: bool, create_new_file: bool) {
    let mut position = 0;

    let _raw = enable_raw_mode().unwrap();

    loop {
        clear_screen();
        print!("{buffer}");

        match read_key().unwrap() {
            Key::Up => println!("Up"),
            Key::Down => println!("Down"),
            Key::Left => println!("Left"),
            Key::Right => println!("Right"),
            Key::Char('q') => break,
            Key::Char(c) => print!("{c}"),
            Key::Esc => println!("Esc"),
            Key::Unknown => {}
        }
    }
}
