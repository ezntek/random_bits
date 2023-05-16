use clap::Parser;
use colored::Colorize;
use meow_translator::{*, translate::*};

fn main() {
    let args = Cli::parse();

    match args.action {
        Command::Encode { text } => {
            let result = encode(&text);
            println!("{0} {1}", "Result: ".bold(), result) 
        },
        Command::Decode { text } => {
            let result = decode(&text);
            println!("{0} {1}", "Result: ".bold(), result)
        },
    }
}