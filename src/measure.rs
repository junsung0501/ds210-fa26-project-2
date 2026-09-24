//! Running many games and reporting what happened.

use crate::secret_keeper::Dealer;
use crate::strategies::{Approach, bad, binary, clever, jump, linear, lucky, random};

/// What a run of games revealed about a strategy.
#[derive(Debug)]
pub struct Results {
    /// The fewest questions any single game needed.
    pub best: u32,
    /// The average across every game.
    pub mean: f64,
    /// The most any single game needed.
    pub worst: u32,
    /// The standard deviation: how spread out the games were around the mean.
    pub std_dev: f64,
}

impl Results {
    /// Is this strategy better than `other`?
    /// **You decide what better means**, using any of the four fields, or several of them.
    /// There is no single right answer (though there are wrong ones), so make a choice
    /// you can defend.
    pub fn is_better_than(&self, other: &Results) -> bool {
        // YOUR SOLUTION GOES HERE.
        todo!("is_better_than")
    }
}

/// Play `rounds` games of `approach` on the range `[min, max)` and report how many
/// questions they took. Every game is dealt from one `Dealer`, so no secret
/// repeats until the whole range has been used.
pub fn measure(approach: Approach, min: u32, max: u32, rounds: u32) -> Results {
    // YOUR SOLUTION GOES HERE.
    todo!("measure")
}
