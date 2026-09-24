//! Every test in the project, grouped by the checkpoint it belongs to.
//!
//!     cargo test --bin game cp1     your bashcrawl answers
//!     cargo test --bin game cp2     do your strategies work, and work as asked?
//!     cargo test --bin game cp3     everything else
//!
//! You do not write any of these, but read them. They say what each strategy is
//! supposed to cost, and the final autograder runs cases that are not here.

// ===========================================================================
// Checkpoint 1
// ===========================================================================

mod cp1 {
    use crate::dungeon::{AVIARY_ANIMAL, FINAL_BOSS, NURSERY_NAME, STRONGHOLD_OBJECT, TEACHER};
    use std::fs;
    use std::sync::LazyLock;
    use std::path::Path;
    use regex::Regex;

    // Statically and lazily stores the log contents as a string
    pub(super) static LOG_CONTENTS: LazyLock<String> = LazyLock::new(|| {
        // Uses the compile time env that Cargo sends the project to as path for reliability
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("dungeon_transcript.txt");
        fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("Failed to read log file at {:?}: {}.", path, e))
    });


    // The answers are stored as hashes so that reading this file does not 
    // mean you can skip bashcrawl, but you can still check your own answers

    /// FNV-1a, 64 bit. Small enough to read, and it gives the same number on
    /// every machine and every version of Rust
    fn fnv1a(s: &str) -> u64 {
        let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
        for byte in s.as_bytes() {
            hash ^= *byte as u64;
            hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
        }
        hash
    }

    /// Lowercase, trim, squeeze spaces, drop leading "the" and ending punctuation
    fn normalize(s: &str) -> String {
        let lowered = s.trim().to_lowercase();
        let trimmed = lowered.trim_end_matches(|c| ".!?,;:".contains(c));
        let mut words: Vec<&str> = trimmed.split_whitespace().collect();
        if words.len() > 1 && words[0] == "the" {
            words.remove(0);
        }
        words.join(" ")
    }

    fn check(name: &str, question: &str, answer: &str, want: u64) {
        let normalized = normalize(answer);
        assert!(
            !normalized.is_empty(),
            "{name} in src/dungeon.rs is still empty.\n  {question}"
        );
        assert!(
            fnv1a(&normalized) == want,
            "{name} in src/dungeon.rs is not the answer we are looking for.\n  \
             {question}\n  \
             You wrote: {answer:?}\n  \
             Capitalization and spaces are ignored, so check the spelling, and if you \
             are sure you are right, ask a TA/CA who all have access to answers."
        );
    }

    /// Counts the lines where `cmd` was typed at a shell prompt (once per line)
    /// Command has to follow a prompt symbol so we don't count text in files
    fn count_command(cmd: &str) -> usize {
        let pattern = format!(r"(?m)^(?:.*[$%>]\s*)?\s*{}\b", regex::escape(cmd));
        let re = Regex::new(&pattern).unwrap();
        re.find_iter(&LOG_CONTENTS).count()
    }

    /// One command's worth of the checks below.
    fn check_command(cmd: &str, wanted: usize) {
        let found = count_command(cmd);
        assert!(
            found >= wanted,
            "dungeon_transcript.txt shows `{cmd}` typed {found} time(s), and we are \
             looking for at least {wanted}. Check that your whole session is pasted in, \
             and if it is, go back into the dungeon and use `{cmd}` some more."
        );
    }

    #[test]
    fn teacher() {
        check(
            "TEACHER",
            "Who taught you to read the runes? Include the epithet.",
            TEACHER,
            0x2270_dfce_b851_09b7,
        );
    }

    #[test]
    fn aviary_animal() {
        check(
            "AVIARY_ANIMAL",
            "What waddles around the aviary? One word, singular.",
            AVIARY_ANIMAL,
            0x8f79_0dee_72e8_95df,
        );
    }

    #[test]
    fn nursery_name() {
        check(
            "NURSERY_NAME",
            "What does the writing on the wall call the nursery? Five words.",
            NURSERY_NAME,
            0x6b8d_3766_ac6a_ef85,
        );
    }

    #[test]
    fn stronghold_object() {
        check(
            "STRONGHOLD_OBJECT",
            "Which object in the stronghold do you free by adding orbs? One word.",
            STRONGHOLD_OBJECT,
            0xeb57_ba9b_fbd5_599c,
        );
    }

    #[test]
    fn final_boss() {
        check(
            "FINAL_BOSS",
            "What is waiting in the pit? One word.",
            FINAL_BOSS,
            0xb62f_733b_2f72_df4e,
        );
    }

    #[test]
    fn ls_count_at_least_5() {
        check_command("ls", 5);
    }

    #[test]
    fn cd_count_at_least_5() {
        check_command("cd", 5);
    }

    #[test]
    fn cat_count_at_least_5() {
        check_command("cat", 5);
    }

    #[test]
    fn ln_at_least_1() {
        check_command("ln", 1);
    }


}


// ===========================================================================
// Checkpoint 2: four strategies that work
// ===========================================================================
//
// Two questions per strategy: does it find the number, and does it find it the
// way we asked? Four copies of the same search would answer the first question
// and miss the point of the project, so the second half of this module counts
// the questions each one spends.

mod cp2 {
    use crate::secret_keeper::SecretKeeper;
    use crate::strategies::{binary, jump, linear, lucky};

    /// Run a strategy against every number in a range and check it finds each one.
    fn finds_every_number(strategy: fn(&mut SecretKeeper, u32, u32) -> u32, min: u32, max: u32) {
        for number in min..max {
            let mut keeper = SecretKeeper::known(number);
            assert_eq!(
                strategy(&mut keeper, min, max),
                number,
                "failed on {number} in [{min}, {max})"
            );
        }
    }

    // -----------------------------------------------------------------------
    // linear
    // -----------------------------------------------------------------------

    #[test]
    fn linear_finds_every_number() {
        finds_every_number(linear, 0, 40);
    }

    /// A range that does not start at zero. This is the most common bug in the
    /// project: code that starts counting at 0 instead of at `min`.
    #[test]
    fn linear_handles_a_shifted_range() {
        finds_every_number(linear, 70, 80);
    }

    // -----------------------------------------------------------------------
    // binary
    // -----------------------------------------------------------------------

    #[test]
    fn binary_finds_every_number() {
        finds_every_number(binary, 0, 40);
    }

    #[test]
    fn binary_handles_a_shifted_range() {
        finds_every_number(binary, 70, 80);
    }

    // -----------------------------------------------------------------------
    // jump
    // -----------------------------------------------------------------------

    #[test]
    fn jump_finds_every_number() {
        finds_every_number(jump, 0, 40);
    }

    #[test]
    fn jump_handles_a_shifted_range() {
        finds_every_number(jump, 70, 80);
    }

    // -----------------------------------------------------------------------
    // lucky
    // -----------------------------------------------------------------------

    #[test]
    fn lucky_finds_every_number() {
        finds_every_number(lucky, 0, 40);
    }

    #[test]
    fn lucky_handles_a_shifted_range() {
        finds_every_number(lucky, 70, 80);
    }

    // -----------------------------------------------------------------------
    // What each strategy costs
    // -----------------------------------------------------------------------
    //
    // These separate a strategy that is correct from the one we asked for.

    /// The most questions a strategy needs for any number in `[min, max)`.
    fn worst_case(strategy: fn(&mut SecretKeeper, u32, u32) -> u32, min: u32, max: u32) -> u32 {
        let mut worst = 0;
        for number in min..max {
            let mut keeper = SecretKeeper::known(number);
            strategy(&mut keeper, min, max);
            if keeper.questions_asked() > worst {
                worst = keeper.questions_asked();
            }
        }
        worst
    }

    /// One question per number, including the last one.
    #[test]
    fn linear_costs_one_question_per_number() {
        assert_eq!(
            worst_case(linear, 0, 100),
            100,
            "linear should ask about every number in the range, one at a time"
        );
    }

    /// Seven in the worst case. Some numbers come out in six, because 100 is not
    /// a power of two.
    #[test]
    fn binary_costs_log_of_the_range() {
        assert_eq!(
            worst_case(binary, 0, 100),
            7,
            "seven questions are enough to halve a hundred numbers down to one. \
             More than that and binary is not throwing away half the range each \
             time it asks"
        );
    }

    /// Between the other two. The exact figure depends on the `STRIDE` you chose.
    #[test]
    fn jump_costs_about_the_square_root_of_the_range() {
        assert!(
            worst_case(jump, 0, 100) <= 30,
            "jump took {} questions in the worst case. Is STRIDE still 1?",
            worst_case(jump, 0, 100)
        );
    }

    /// Twice binary's: one extra question per level.
    #[test]
    fn lucky_costs_twice_what_binary_does() {
        assert!(
            worst_case(lucky, 0, 100) <= 14,
            "lucky took {} questions in the worst case, and it should need about \
             twice what binary does. It spends one extra question per level, not \
             one extra question per number",
            worst_case(lucky, 0, 100)
        );
    }
}

// ===========================================================================
// Checkpoint 3
// ===========================================================================

mod cp3 {
    use crate::measure::{measure, Results};
    use crate::secret_keeper::{Dealer, SecretKeeper};
    use crate::strategies::{binary, clever, first_possible, possible_count, Approach};

    // -----------------------------------------------------------------------
    // measure
    // -----------------------------------------------------------------------

    /// A full sweep meets every number exactly once, so the hundred games cost
    /// 1 through 100 questions in some order.
    #[test]
    fn measure_reports_a_full_sweep_of_linear() {
        let r = measure(Approach::Linear, 0, 100, 100);
        assert_eq!(r.best, 1, "the easiest game is the one where the number is min");
        assert_eq!(r.worst, 100, "the hardest game is the one where the number is max - 1");
        assert!(
            (r.mean - 50.5).abs() < 0.01,
            "mean of 1 through 100 is 50.5, got {}",
            r.mean
        );
        assert!(
            r.std_dev > 28.0 && r.std_dev < 29.5,
            "standard deviation of 1 through 100 is about 28.9, got {}. \
             If you got about 833, you forgot the square root",
            r.std_dev
        );
    }

    /// On a range of exactly 64, binary splits evenly every time, so every game
    /// costs the same and there is no spread at all.
    #[test]
    fn measure_reports_a_strategy_with_no_spread() {
        let r = measure(Approach::Binary, 0, 64, 64);
        assert_eq!(r.best, 6);
        assert_eq!(r.worst, 6);
        assert!((r.mean - 6.0).abs() < 0.01, "expected a mean of 6.0, got {}", r.mean);
        assert!(
            r.std_dev < 0.01,
            "every game cost the same, so the spread is 0. Got {}",
            r.std_dev
        );
    }

    /// Whatever the games cost, the fewest cannot be more than the average and
    /// the average cannot be more than the most. True of any correct `measure`,
    /// on any strategy, which is the point: this one does not check your numbers,
    /// it checks that they are describing the same set of games.
    #[test]
    fn measure_reports_a_best_mean_and_worst_that_agree() {
        let r = measure(Approach::Jump, 0, 100, 100);
        assert!(
            r.best as f64 <= r.mean,
            "best was {} but the mean was {}. Did `best` start at 0?",
            r.best,
            r.mean
        );
        assert!(
            r.mean <= r.worst as f64,
            "the mean was {} but the worst game was {}",
            r.mean,
            r.worst
        );
    }

    /// One game played means one number in the set, so the fewest, the average
    /// and the most are all that same number, and nothing is spread out.
    #[test]
    fn measure_of_a_single_round_has_no_spread() {
        let r = measure(Approach::Binary, 0, 100, 1);
        assert_eq!(
            r.best, r.worst,
            "only one game was played, so the best and worst game are the same game"
        );
        assert!(
            (r.mean - r.best as f64).abs() < 0.01,
            "one game costing {} questions has a mean of {}, got {}",
            r.best,
            r.best,
            r.mean
        );
        assert!(
            r.std_dev < 0.01,
            "one game cannot be spread out around itself. Got {}",
            r.std_dev
        );
    }

    // -----------------------------------------------------------------------
    // is_better_than
    // -----------------------------------------------------------------------

    /// We do not grade your definition of "better", only that it is a definition.
    /// One strategy here wins on all four fields, which no reasonable rule can
    /// call a loss, and a rule that always answers the same thing fails one half
    /// of this test or the other.
    #[test]
    fn is_better_than_calls_the_obvious_case() {
        let great = Results { best: 1, mean: 6.5, worst: 7, std_dev: 1.0 };
        let awful = Results { best: 4, mean: 50.5, worst: 100, std_dev: 28.9 };

        assert!(
            great.is_better_than(&awful),
            "a strategy that wins on best, mean, worst and std_dev is the better one"
        );
        assert!(
            !awful.is_better_than(&great),
            "a strategy beaten on best, mean, worst and std_dev is not the better one. \
             Does your rule ever answer false?"
        );
    }

    // -----------------------------------------------------------------------
    // possible_count and first_possible
    // -----------------------------------------------------------------------

    /// A keeper built by `known` has played nothing, so nothing is ruled out.
    #[test]
    fn possible_count_counts_a_range_with_no_history() {
        let keeper = SecretKeeper::known(7);
        assert_eq!(possible_count(&keeper, 5, 10), 6, "5, 6, 7, 8, 9 and 10 is six numbers");
        assert_eq!(possible_count(&keeper, 3, 3), 1, "both ends are included");
    }

    /// Each keeper knows the secrets from earlier games.
    #[test]
    fn possible_count_shrinks_as_games_are_played() {
        let mut dealer = Dealer::new(0, 10);
        for played in 0..5 {
            let keeper = dealer.deal();
            assert_eq!(
                possible_count(&keeper, 0, 9),
                10 - played,
                "after {played} games, {played} numbers are used up"
            );
        }
    }

    /// With no history the first possible number is just the bottom of the range.
    #[test]
    fn first_possible_finds_the_bottom_of_the_range() {
        let keeper = SecretKeeper::known(7);
        assert_eq!(first_possible(&keeper, 5, 10), Some(5));
        assert_eq!(first_possible(&keeper, 0, 99), Some(0));
    }

    /// Two of three numbers are gone, so there is exactly one answer and it is
    /// probably not the bottom of the range.
    #[test]
    fn first_possible_skips_numbers_already_used() {
        let mut dealer = Dealer::new(0, 3);
        let _ = dealer.deal();
        let _ = dealer.deal();
        let keeper = dealer.deal();

        let only_one_left = (0..3).find(|n| keeper.is_still_possible(*n));
        assert_eq!(first_possible(&keeper, 0, 2), only_one_left);
    }

    /// A range holding nothing but a used number has no answer to give.
    #[test]
    fn first_possible_returns_none_when_there_is_no_answer() {
        let mut dealer = Dealer::new(0, 4);
        let _ = dealer.deal();
        let keeper = dealer.deal();

        let used = (0..4)
            .find(|n| !keeper.is_still_possible(*n))
            .expect("the second game always has one number behind it");
        assert_eq!(first_possible(&keeper, used, used), None);
    }

    // -----------------------------------------------------------------------
    // clever
    // -----------------------------------------------------------------------

    /// With no history to exploit, `clever` still has to find every number.
    #[test]
    fn clever_finds_every_number_with_no_history() {
        for (min, max) in [(0, 40), (70, 80), (5, 6)] {
            for number in min..max {
                let mut keeper = SecretKeeper::known(number);
                assert_eq!(
                    clever(&mut keeper, min, max),
                    number,
                    "failed on {number} in [{min}, {max})"
                );
            }
        }
    }

    /// A whole sweep against a real dealer. This test cannot see the secret, so it
    /// asks the keeper to confirm whatever `clever` handed back.
    #[test]
    fn clever_is_right_even_when_history_narrows_the_range() {
        let mut dealer = Dealer::new(0, 20);
        for round in 0..20 {
            let mut keeper = dealer.deal();
            let answer = clever(&mut keeper, 0, 20);
            assert!(
                keeper.ask_if_equal(answer),
                "in round {round}, clever returned {answer}, which is not the number"
            );
        }
    }

    /// `clever` is the only strategy that improves as a run goes on, so it cannot
    /// be judged one game at a time.
    #[test]
    fn clever_beats_binary_over_a_full_sweep() {
        let plain = measure(Approach::Binary, 0, 100, 100);
        let smart = measure(Approach::Clever, 0, 100, 100);
        assert!(
            smart.mean < plain.mean - 0.5,
            "clever averaged {:.2} questions, binary averaged {:.2}",
            smart.mean,
            plain.mean
        );
        assert_eq!(
            smart.best, 0,
            "the last game of a sweep has one number left, so it costs no questions"
        );
    }

    // -----------------------------------------------------------------------
    // The midpoint
    // -----------------------------------------------------------------------

    /// `binary` on a range at the very top of `u32`. Both ends fit in a `u32`.
    /// Their sum does not.
    #[test]
    fn binary_survives_the_top_of_u32() {
        let min = u32::MAX - 40;
        for number in min..u32::MAX {
            let mut keeper = SecretKeeper::known(number);
            assert_eq!(binary(&mut keeper, min, u32::MAX), number);
        }
    }

    // -----------------------------------------------------------------------
    // The refactor
    // -----------------------------------------------------------------------

    /// The text of one `pub fn` in a source file, with comments removed.
    fn body_of(source: &str, name: &str) -> String {
        let start = source
            .find(&format!("pub fn {name}("))
            .unwrap_or_else(|| panic!("there is no `pub fn {name}` in strategies.rs"));
        let rest = &source[start..];
        let end = rest[1..].find("\npub ").map(|i| i + 1).unwrap_or(rest.len());
        rest[..end]
            .lines()
            .map(|line| line.split("//").next().unwrap_or(""))
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// `jump`'s walking phase is a linear search, so it should call `linear`
    /// rather than hold a second copy. Both versions ask identical questions, so
    /// this reads the source instead of playing.
    #[test]
    fn jump_calls_linear() {
        assert!(
            body_of(include_str!("strategies.rs"), "jump").contains("linear("),
            "jump does not call linear. The second half of jump walks the numbers \
             it skipped, one at a time, asking about each. That is linear. Call the \
             one you already wrote."
        );
    }
}
