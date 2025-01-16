use clap::Parser;

mod cli;
mod indexer;
mod lexer;
mod server;
mod tf_idf;
mod types;

fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();
    match cli.command {
        cli::Commands::Index { dir, index } => indexer::index_folder(&dir, &index)?,
        cli::Commands::Search { index, term: _ } => indexer::check_index(&index)?, // TODO rename
        cli::Commands::Serve { port, index } => server::serve(port, &index)?,
    }
    Ok(())
}
