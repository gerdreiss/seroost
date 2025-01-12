use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    /// Directory to index
    Index {
        #[arg(short, long, value_name = "DIR")]
        dir: PathBuf,
        #[arg(short, long, value_name = "INDEX")]
        index: PathBuf,
    },
    /// Index to search
    Search {
        #[arg(short, long, value_name = "INDEX")]
        index: PathBuf,
        term: String,
    },
}
