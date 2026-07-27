use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    name = "nb",
    version,
    about = "A minimal text editor.",
    long_about = None,
)]

struct Cli {
    path: PathBuf,
}
