//! Checkpoint 1 only.
//!
//! The guessing game itself arrives in the next release of this repo. For now
//! this file is here so that `cargo test --bin game cp1` has something to run.
//! Do not edit it.

// These modules aren't truly used by game.rs but here to enable testing
mod dungeon;
mod version;
#[cfg(test)]
mod tests;

fn main() {
    println!("Checkpoint 1 is the dungeon, not this program.");
    println!();
    println!("Record your session in dungeon_transcript.txt, put your answers in");
    println!("src/dungeon.rs, and check them with:");
    println!();
    println!("    cargo test --bin game cp1");
}
