use clap::{Parser, Subcommand};
mod utils;

#[derive(Parser, Debug)]
#[command(
    name = "zk-utils",
    version = "0.0.1",
    about = "Evan's utilities for managing a traditional zettelkasten"
)]

struct Cli {
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Graph { files: Vec<String> },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Graph { files } => {
            utils::graph::print_json(args.verbose, files);
        }
    }
}
