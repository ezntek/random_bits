pub mod translate;
pub use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum Command {
    Encode { text: String },
    Decode { text: String }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "None")]
pub struct Cli {
    #[command(subcommand)]
    pub action: Command
}