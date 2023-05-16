use clap::Parser;
use meow_translator::{*, translate::*};

fn main() {
    let args = Cli::parse();

    match args.action {
        Command::Encode { text } => {
            let result = encode(&text);
            println!("Output: {}", result) 
        },
        Command::Decode { text } => {
            let result = decode(&text);
            println!("Output: {}", result)
        },
    }
}