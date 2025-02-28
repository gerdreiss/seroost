use clap::Parser;
use clap::Subcommand;
use std::path::PathBuf;

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
    /// Check index
    Check {
        #[arg(short, long, value_name = "INDEX")]
        index: PathBuf,
    },
    /// Start server and load the index
    Serve {
        #[arg(short, long, value_name = "PORT", default_value_t = 8080)]
        port: usize,
        #[arg(short, long, value_name = "INDEX")]
        index: PathBuf,
    },
}
