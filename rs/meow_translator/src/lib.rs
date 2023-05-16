mod translate;

use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
enum Command {
    Encode { text: String },
    Decode { text: String }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "None")]
struct Cli {
    #[command(subcommand)]
    action: Command
}