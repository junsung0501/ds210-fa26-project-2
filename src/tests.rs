//! Every test in the project, grouped by the checkpoint it belongs to.
//!
//!     cargo test --bin game cp1     your bashcrawl answers
//!
//! The tests for the later checkpoints arrive with the rest of the project.
//! You do not write any of these, but you should read them.

// ===========================================================================
// Checkpoint 1
// ===========================================================================

mod cp1 {
    use crate::dungeon::{AVIARY_ANIMAL, FINAL_BOSS, NURSERY_NAME, STRONGHOLD_TREASURE, TEACHER};
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
    fn stronghold_treasure() {
        check(
            "STRONGHOLD_TREASURE",
            "Which object in the stronghold has to have its material form freed? One word.",
            STRONGHOLD_TREASURE,
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


