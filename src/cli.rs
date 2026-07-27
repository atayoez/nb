use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    name = "nb",
    version,
    about = "A minimal text editor.",
    long_about = None,
)]

pub(crate) struct Cli {
    pub(crate) path: Option<PathBuf>,
}
