use crate::flags::Flags;
use std::fs::File;
use std::io;
use std::io::{Read, Write};

pub struct Editor {
    file: Option<File>,
    buffer: String,
    current: usize,
    flags: Flags,
}

impl Editor {
    pub fn new(flags: Flags) -> Self {
        if !flags.create_new_file {
            let mut file = File::open(flags.clone().path.unwrap()).unwrap();
            let mut buffer = String::new();
            file.read_to_string(&mut buffer).unwrap();

            Editor {
                file: Some(file),
                buffer,
                current: 0,
                flags,
            }
        } else {
            Editor {
                file: None,
                buffer: String::new(),
                current: 0,
                flags,
            }
        }
    }

    pub fn display(&self) {
        let mut out = io::stdout().lock();
        // Clear screen
        print!("\x1b[2J\x1b[H");
        for c in self.buffer.chars() {
            write!(out, "{c}").unwrap();
        }
        out.flush().unwrap();
    }
}
