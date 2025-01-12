use clap::Parser;

mod cli;
mod indexer;
mod lexer;

fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();
    match cli.command {
        cli::Commands::Index { dir, index } => indexer::index_folder(&dir, &index)?,
        cli::Commands::Search { index, term: _ } => indexer::check_index(&index)?,
    }

    Ok(())
}
