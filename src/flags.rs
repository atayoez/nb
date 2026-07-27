use crate::cli::Cli;
use std::path::PathBuf;

pub struct Flags {
    create_new_file: bool,
    path: Option<PathBuf>,
    saved: bool,
}

impl Flags {
    pub fn from(cli: Cli) -> Self {
        match cli.path {
            Some(path) => Flags {
                create_new_file: false,
                path: Some(path),
                saved: false,
            },
            None => Flags {
                create_new_file: true,
                path: None,
                saved: false,
            },
        }
    }
}
