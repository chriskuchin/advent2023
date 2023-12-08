use clap::{Parser, Subcommand};
use std::time::Instant;

mod day1;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Day1 {},
    Day2 {},
    Day3 {},
}

fn main() {
    let cli = Cli::parse();

    let start = Instant::now();
    match cli.command {
        Some(Commands::Day1 {}) => day1::solve(),
        Some(Commands::Day2 {}) => println!("unsolved"),
        Some(Commands::Day3 {}) => println!("unsolved"),
        None => println!("failed"),
    }
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
