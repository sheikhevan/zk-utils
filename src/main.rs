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
    /// Generate a new json file for use in a graph view (like zk-graph)
    Graph {
        #[arg(short, long)]
        /// The markdown files or folder that contains said files
        input: Vec<String>,

        /// The file you want the outputted json to go to
        #[arg(short, long)]
        output: String,
    },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Graph { input, output } => {
            utils::graph::print_json(args.verbose, input);
        }
    }
}
