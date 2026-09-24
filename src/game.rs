//! Play one game against the computer, with you keeping the secret.
//!
//! Pick a strategy with `--strategy`, and the computer will ask you questions
//! until it works out your number. This is the quickest way to see what a
//! strategy you have just written actually does.
//!
//!     cargo run --bin game -- --strategy binary --min 0 --max 64

use std::io;
use clap::Parser;
use crate::secret_keeper::SecretKeeper;
use crate::strategies::{bad, binary, clever, jump, linear, lucky, random, Approach};

// These modules aren't truly used by game.rs but here to enable testing
mod dungeon;
mod measure;
mod version;
#[cfg(test)]
mod tests;
// These, however, are used:
mod secret_keeper;
mod strategies;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    strategy: Approach,
    #[arg(long, default_value_t = 0)]
    min: u32,
    #[arg(long, default_value_t = 16)]
    max: u32,
}

fn main() {
    // Read the command line arguments.
    let cli = Args::parse();
    let strategy = cli.strategy;
    let min = cli.min;
    let max = cli.max;

    // The person at the keyboard is the one keeping the secret.
    let mut keeper = SecretKeeper::human();

    println!("Choose a number between {min} (inclusive) and {max} (exclusive).");
    println!("Write down your number on a piece of paper so you remember what it is.");
    println!("Do not tell me what your number is. I will try to guess it.");
    println!("Did you choose a number? Hit enter when you have! ");

    // Wait until user types in enter.
    let _ = String::new();
    io::stdin().read_line(&mut String::new()).unwrap();

    // Game starts.
    println!("Commencing game!");
    let answer = match strategy {
        Approach::Bad => bad(&mut keeper, min, max),
        Approach::Random => random(&mut keeper, min, max),
        Approach::Linear => linear(&mut keeper, min, max),
        Approach::Binary => binary(&mut keeper, min, max),
        Approach::Jump => jump(&mut keeper, min, max),
        Approach::Lucky => lucky(&mut keeper, min, max),
        Approach::Clever => clever(&mut keeper, min, max),
    };

    // Print results!
    println!("Final answer: {answer}");
    println!("Took {} questions", keeper.questions_asked());
}
