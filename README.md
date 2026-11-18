# my_first_project

My first real Rust project — a hands-on companion to the exercise workspace
on the [`main` branch](https://github.com/kodix75/Rust-Exercise).

The idea: finish the ten exercises, then build **one** real program here
that uses everything they teach (structs, enums, `Result`, file I/O,
arguments, `Vec`, `match`).

The full guide — setup, syntax cheat-sheet, all ten exercises, project
ideas, common errors and commands — is in
**[`RUST-EXERCISES.md`](RUST-EXERCISES.md)**.

---

## Status

```markdown
## Environment
- OS: Fedora 44
- Rust: 1.99.1 (Fedora package, sysroot /usr)
- rust-analyzer: VS Code extension
- Project: /home/aidan/Projects/Github/my_first_project

## Setup
- [x] cargo new / cargo run / cargo check
- [x] rust-analyzer sysroot fixed (Fedora `rust-src`)
- [ ] Read the syntax cheat-sheet

## Exercises
- [ ] Ex 1  — print & math
- [ ] Ex 2  — mutability & shadowing
- [ ] Ex 3  — FizzBuzz
- [ ] Ex 4  — factorial
- [ ] Ex 5  — Vec statistics
- [ ] Ex 6  — guess the number ⭐
- [ ] Ex 7  — BankAccount struct
- [ ] Ex 8  — enum calculator
- [ ] Ex 9  — read a file
- [ ] Ex 10 — CLI args

## First real project
- [ ] Chosen: ___________
- [ ] Working
- [ ] Polished (errors handled, README updated)
```

## Run it

```bash
cargo run                                  # this project (src/main.rs)
cargo check                                # type-check
cargo fmt                                  # format
cargo clippy                               # lints
```

## Where the code goes

```
my_first_project/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── RUST-EXERCISES.md   # the learning pack
└── src/
    ├── main.rs         # the real project
    └── bin/            # exercises, one file each (create this)
        ├── ex01.rs
        └── …
```

```bash
cargo run --bin ex01     # run an exercise
```

## Notes

- `target/` is ignored by git.
- Each exercise: create the file, run it clean (no warnings), tick it
  above, commit.
- Commit style: `ex01: print & math`.

## Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [std docs](https://doc.rust-lang.org/std/)
