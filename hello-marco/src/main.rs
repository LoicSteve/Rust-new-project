// command-line tool to play Marco Polo

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version = "1.0", author = "Loic Steve", about = "Marco Polo game")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(version = "1.0", author = "Loic Steve")]
    Play {
        #[arg(short, long)]
        name: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Play { name }) => {
            let result = hello_marco::marco(&name);
            println!("{}", result);
        }
        None => {
            println!("What is your name?");
        }
    }
}
