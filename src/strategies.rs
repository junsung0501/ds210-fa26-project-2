//! Every guessing strategy in the project goes here.
//!
//! A strategy plays one whole game. It is handed a secret keeper and a range, it
//! asks whatever questions it likes, and it returns the number it has worked out.
//! They differ in which questions they ask.
//!
//! Two types of questions are available:
//!
//!   keeper.ask_if_equal(n)     is it exactly n?
//!   keeper.ask_if_greater(n)   is it bigger than n? (no means less or equal)

use crate::secret_keeper::SecretKeeper;
use clap::ValueEnum;
use rand::random_range;

/// Names the strategies available (among other things, prevents typos!)
#[derive(ValueEnum, Clone)]
pub enum Approach {
    Bad,
    Random,
    Linear,
    Binary,
    Jump,
    Lucky,
    Clever,
}

// ---------------------------------------------------------------------------
// Worked examples
// ---------------------------------------------------------------------------

/// Ask whether the number is the smallest in the range, and if it is not, assume
/// it is the largest. This is wrong almost all the time.
pub fn bad(keeper: &mut SecretKeeper, min: u32, max: u32) -> u32 {
    if keeper.ask_if_equal(min) {
        min
    } else {
        max - 1
    }
}

/// Guess at random until the guess happens to be right.
/// Always correct, eventually, but could take arbitrarily long.
pub fn random(keeper: &mut SecretKeeper, min: u32, max: u32) -> u32 {
    loop {
        let guess = random_range(min..max);
        if keeper.ask_if_equal(guess) {
            return guess;
        }
    }
}

// ---------------------------------------------------------------------------
// Search strategies
// ---------------------------------------------------------------------------

/// Try `min`, then `min + 1`, then `min + 2`, until one of them is the number.
///
/// Honest answers mean one of them always is. Decide what to do if the loop ends
/// anyway, and say why: see "What if you run out of numbers?" in the handout.
pub fn linear(keeper: &mut SecretKeeper, min: u32, max: u32) -> u32 {
    // YOUR SOLUTION GOES HERE.
    todo!("linear")
}

/// Halve the range at each step: ask whether the number is above the midpoint,
/// discard the half that cannot contain it, and repeat.
pub fn binary(keeper: &mut SecretKeeper, min: u32, max: u32) -> u32 {
    // YOUR SOLUTION GOES HERE.
    todo!("binary")
}

/// How far `jump` moves on each step forward.
pub const STRIDE: u32 = 1;

/// Step forward `STRIDE` at a time until the number is behind you, then walk back
/// through the numbers you skipped.
///
/// Like `linear`, this should always find the number inside the loop, but the
/// compiler can't know that, so you still have to say what happens if the loop ends
pub fn jump(keeper: &mut SecretKeeper, min: u32, max: u32) -> u32 {
    // YOUR SOLUTION GOES HERE.
    todo!("jump")
}

/// Halve the range like `binary`, but spend one extra question at each step
/// asking outright whether the midpoint is the number. Finishes in a single
/// question when it guesses right on the first try.
pub fn lucky(keeper: &mut SecretKeeper, min: u32, max: u32) -> u32 {
    // YOUR SOLUTION GOES HERE.
    todo!("lucky")
}

// ---------------------------------------------------------------------------
// Search with memory
// ---------------------------------------------------------------------------

/// How many numbers between `lo` and `hi`, both included, could still be the
/// secret?
pub fn possible_count(keeper: &SecretKeeper, lo: u32, hi: u32) -> u32 {
    // YOUR SOLUTION GOES HERE.
    todo!("possible_count")
}

/// The first number in `lo..=hi` that could still be the secret. `None` when
/// every number in the range has already been used.
pub fn first_possible(keeper: &SecretKeeper, lo: u32, hi: u32) -> Option<u32> {
    // YOUR SOLUTION GOES HERE.
    todo!("first_possible")
}

/// Halve the range like `binary`, but stop as soon as only one number in it is
/// still possible. A dealer never repeats a secret until it has used every
/// number, so the more games you play, the more of the range is ruled out.
pub fn clever(keeper: &mut SecretKeeper, min: u32, max: u32) -> u32 {
    // YOUR SOLUTION GOES HERE.
    todo!("clever")
}
