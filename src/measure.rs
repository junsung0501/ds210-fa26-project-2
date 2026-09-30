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
        self.mean < other.mean
    }
}

/// Play `rounds` games of `approach` on the range `[min, max)` and report how many
/// questions they took. Every game is dealt from one `Dealer`, so no secret
/// repeats until the whole range has been used.
pub fn measure(approach: Approach, min: u32, max: u32, rounds: u32) -> Results {
    if rounds == 0 {
        return Results { best: 0, mean: 0.0, worst: 0, std_dev: 0.0 };
    }

    let mut dealer = Dealer::new(min, max);
    let mut best = u32::MAX;
    let mut worst = 0;
    let mut sum = 0.0;
    let mut sum_sq = 0.0;

    for _ in 0..rounds {
        let mut keeper = dealer.deal();
        match approach {
            Approach::Bad => bad(&mut keeper, min, max),
            Approach::Random => random(&mut keeper, min, max),
            Approach::Linear => linear(&mut keeper, min, max),
            Approach::Binary => binary(&mut keeper, min, max),
            Approach::Jump => jump(&mut keeper, min, max),
            Approach::Lucky => lucky(&mut keeper, min, max),
            Approach::Clever => clever(&mut keeper, min, max),
        };
        let count = keeper.questions_asked();
        best = best.min(count);
        worst = worst.max(count);
        sum += count as f64;
        sum_sq += (count as f64) * (count as f64);
    }

    let n = rounds as f64;
    let mean = sum / n;
    let variance = (sum_sq / n) - (mean * mean);

    Results { best, mean, worst, std_dev: variance.max(0.0).sqrt() }
}
