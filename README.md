# Project 1: Guessing Game

**Author:** _johnlee_



### Getting started

Run the simplest version of this game, then follow the instructions on the screen.

```bash
cargo run --bin game -- --strategy random
```

You choose a number and write it down. The computer asks you questions until it guesses
it. The interesting part is how many questions it needs.

Try a few configurations:

```bash
cargo run --bin game -- --strategy bad --min 2 --max 6
cargo run --bin game -- --help
```

These five will fail with `not yet implemented`, because writing them is your job:

```bash
cargo run --bin game -- --strategy linear
cargo run --bin game -- --strategy binary
cargo run --bin game -- --strategy jump
cargo run --bin game -- --strategy lucky
cargo run --bin game -- --strategy clever
```

### The files

```
src/
  game.rs           play a game yourself against the computer
  plot.rs           run every strategy, draw the plot, print the table
  secret_keeper.rs  the other side of the game: who knows the number, and answers
  strategies.rs     every guessing strategy. Five of them are yours, plus two helpers
  measure.rs        run many games and report what happened. This one is yours
  dungeon.rs        your answers from checkpoint 1
  tests.rs          every test in the project
  version.rs        which release of the stencil you have. Ignore it
dungeon_transcript.txt   your bashcrawl session goes in here
```

Most of what you have to write is marked `todo!` and will not compile past it. The handout
says which ones belong to which checkpoint.

**Change only what the handout asks for**: the functions marked `todo!`, the `STRIDE`
constant, the answers in `dungeon.rs`, your session in `dungeon_transcript.txt`, and the
writeup at the bottom of this file. Leave everything else alone.

### About the warnings

A fresh clone builds with a lot of warnings, all of them some version of "you declared this and never used it." That is expected, since you haven't written the code yet.

### Running your tests

All the tests live in `src/tests.rs`, grouped by checkpoint. Run one group:

```bash
cargo test --bin game cp1     # your bashcrawl answers
cargo test --bin game cp2     # do your strategies work, and work as asked?
cargo test --bin game cp3     # everything else
cargo test --bin game         # all of them
```

The filter is a substring of the test's name, so `cargo test --bin game cp2::binary` and
`cargo test --bin game shifted` both work too.

You do not write any of these. A fresh clone fails every one of them, and they go green as
you fill in the `todo!`s.

### The benchmark

```bash
cargo run --bin plot
```

This writes `plot.png` in the project folder and prints the summary table underneath it.
You need both for the writeup.

## Your writeup

Answer the questions from the project handout here, under the headings below. Leave the
headings where they are, and leave everything above this line alone: it is the guide to the
repo and the graders read it too.

Two to three sentences per question unless the handout says otherwise.

### 1. Which strategy is best

Binary is the best strategy shown on plot.png, since clever does not even appear there, and binary's line stays the lowest and flattest, needing only about 7 questions at max 100. My is_better_than function uses the lower mean, and ranking all six gives clever, binary, lucky, jump, linear, random, which matches the plot closely since binary comes right after clever. They do not match exactly just because the plot cannot show clever at all.

### 2. Linear, as a function of n

The linear function checks min, then min plus 1, one number at a time, so in the worst case it checks every number before finding the right one, which is O(n). If max is 110 it needs 110 questions, and if max is 120 it needs 120, matching the test linear_costs_one_question_per_number. So as a function of n, the worst case is just n.

### 3. Binary, as a function of n

Every time binary runs its loop, it cuts the range about in half, so the number of questions is how many times you can cut n in half before one number is left, which is O(log n). The steps happen right after n passes a power of 2: at 2, 3, 5, 9, 17, 33, 65, with the next at 129. This matches the handout's own example, where finding 73 in a range of 100 took exactly 7 questions.

### 4. Jump: its shape, and the stride you picked

Jump's line is a step pattern, but with much smaller and more frequent steps than binary's wide plateaus. This matches the code, since jump moves forward by STRIDE asking if the number is greater, taking about max divided by STRIDE steps before overshooting, then calling linear on the skipped part, which can cost up to STRIDE more questions. We picked STRIDE equals 10, and testing STRIDE equals 30 instead, jump's mean rose from 12.71 to 18.47 and its worst case from 21 to 35, since a bigger stride means fewer jumps but a pricier walk back.

### 5. Lucky vs binary

Lucky asks one extra question at every step, checking if the middle number is exactly the answer, but when the answer is no, that question only removes one number, and lucky still asks the greater than question right after, same as binary. So that extra question is wasted almost every time, only paying off on the rare game lucky guesses right away. This is why lucky's mean of 10.59 is much higher than binary's 6.72, even with a better best case of 1.

### 6. The best column

Binary's first move is always a greater than question about the middle of the range, and that kind of question can only narrow things down, never land on the exact number, which is why its best case is 6, not 1. Jump's first move is the same kind of question, so it cannot get 1 either, its best case is 2. Linear, lucky, and random all start by asking if the number is exactly something, so each has a chance to get lucky and finish in one question.

### 7. Would ask_if_even beat binary

No, I do not think ask_if_even would beat binary search, since binary already asks close to the best possible question every time, reaching the O(log n) lower bound. Any yes or no question can only remove at most half the remaining numbers no matter what it is, so ask_if_even could only tie that limit, not beat it.

### 8. Why clever is not on the plot

Looking at plot.rs, its worst_case function builds a fresh keeper for every trial number, and its own comment says nothing is remembered between calls, which is exactly what clever needs to work. Both possible_count and first_possible read the already_used list, which only grows when real games share one dealer, so if clever ran through worst_case, it would act exactly like binary, with no history to use. Clever's real advantage only shows up in the measure table, which does share one dealer across all 1,000 rounds, and that is why plot.rs leaves it out.

### 9. Something that failed at first, and how you adapted

My first try at avoiding overflow in binary's midpoint was lo plus hi minus 1, divided by 2, which looked safe but was not, since Rust does the addition first, so that first sum alone could overflow near the top of u32's range. This passed all my earlier tests until a new test, binary_survives_the_top_of_u32, caught it. I fixed it by subtracting first instead, lo plus hi minus 1 minus lo divided by 2, so the numbers never get big enough to overflow.

### 10. The September 21 merge

The conflict was in dungeon.rs, where the update renamed STRONGHOLD_TREASURE to STRONGHOLD_OBJECT with a clearer description, but their side was empty, while mine still had the real answer, goblet, under the old name. I kept the new name since it was a real improvement, but kept my own value since it actually came from playing the game. I checked it by running cargo test --bin game cp1 again and seeing all 9 tests pass, instead of just assuming the merge worked.

### AI citation

I used Claude for the project to help write and debug code throughout the Rust and terminal work. For example, I used it to write the first drafts of my strategy functions and the measuring code, which I tested and ran myself with cargo test after every change. I used it to debug three real issues along the way, a stride value, an overflow bug, and a file edit mistake, and to work through resolving the git merge conflict. Claude proposed choices like the rule used in is_better_than and which side of the merge conflict to keep, and I reviewed and understood the reasoning behind both.
