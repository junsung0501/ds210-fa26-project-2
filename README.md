# Project 1: Guessing Game

**Author:** _your name here_



### What is here so far

This repo arrives in two pieces. Right now it holds checkpoint 1, which is the bashcrawl dungeon. The guessing game itself, and the tests for it, will come in an update you will merge in about a week.

```
src/
  dungeon.rs        your answers from checkpoint 1. This one is yours to edit
  tests.rs          the checkpoint 1 tests
  game.rs           a placeholder until the real game arrives
  version.rs        which release of the stencil you have. Ignore it
dungeon_transcript.txt   your bashcrawl session goes in here
```

**Change only what the handout asks for**: the answers in `dungeon.rs`, your session in `dungeon_transcript.txt`, and the Author line at the top of this file. Leave everything else alone for now.

### Running your tests

```bash
cargo test --bin game cp1
```

That checks your five answers and that you pasted a session into `dungeon_transcript.txt`. A fresh clone fails all of them and they go green as you fill them in. The last word in the command is a substring of the test name, so `cargo test --bin game teacher` runs just that one test.

The first `cargo run` or `cargo test` line has to build the project's dependencies and takes a couple of minutes. After that it should be quick.

### About the warnings

The build prints a lot of warnings like "you declared this and never used it." That is expected, since the code that uses them is coming in an updated version of the project.
