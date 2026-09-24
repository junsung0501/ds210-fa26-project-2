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

### 2. Linear, as a function of n

### 3. Binary, as a function of n

### 4. Jump: its shape, and the stride you picked

### 5. Lucky vs binary

### 6. The best column

### 7. Would `ask_if_even` beat binary

### 8. Why `clever` is not on the plot

### 9. Something that failed at first, and how you adapted

### 10. The September 21 merge

### AI citation
