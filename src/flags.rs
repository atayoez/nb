use crate::cli::Cli;
use std::path::PathBuf;

#[derive(Clone)]
pub struct Flags {
    pub create_new_file: bool,
    pub(crate) path: Option<PathBuf>,
    saved: bool,
}
impl Flags {
    pub fn new(cli: Cli) -> Self {
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
