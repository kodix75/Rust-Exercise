# Rust Exercises

A hands-on Rust learning workspace: **one small runnable program per
exercise**, built while working through the language fundamentals on Fedora.

The first real project lives on its own branch:
[`my_first_project`](https://github.com/kodix75/Rust-Exercise/tree/my_first_project).

---

## Layout

```
Rust-Exercise/
├── Cargo.toml          # package: rust-exercise
├── Cargo.lock
├── README.md           # this file
└── src/
    ├── main.rs         # placeholder entry point
    └── bin/            # one file per exercise
        └── ex01.rs     # (created as you go)
```

Everything Rust-related lives in `src/`. Each file in `src/bin/` is a
separate binary with its own `fn main()`.

## Requirements

Built and tested with Fedora's system Rust.

```bash
sudo dnf install rust cargo rust-src clippy
rustc --version    # 1.99.1
cargo --version    # 1.99.1
```

`rust-src` is what rust-analyzer needs to load the standard library on
Fedora (sysroot `/usr`). If the editor complains about a missing sysroot:

```bash
ls /usr/lib/rustlib/src/rust/library/core/src/lib.rs
echo $RUST_SRC_PATH        # must be empty
sudo dnf reinstall rust-src
```

## Quick start

```bash
# add an exercise
$EDITOR src/bin/ex01.rs

# run it
cargo run --bin ex01

# type-check only, warnings included
cargo check --bin ex01

# lint
cargo clippy --bin ex01

# format
cargo fmt
```

## Exercises

- [ ] **Ex 1** — print & math
- [ ] **Ex 2** — mutability & shadowing
- [ ] **Ex 3** — FizzBuzz
- [ ] **Ex 4** — factorial (loop + recursion)
- [ ] **Ex 5** — Vec statistics
- [ ] **Ex 6** — guess the number
- [ ] **Ex 7** — `BankAccount` struct
- [ ] **Ex 8** — enum calculator
- [ ] **Ex 9** — read a file
- [ ] **Ex 10** — CLI arguments

The full descriptions, starters and hints are in the learning pack on the
project branch:
[`RUST-EXERCISES.md`](https://github.com/kodix75/Rust-Exercise/blob/my_first_project/RUST-EXERCISES.md).

## Branches

| Branch | Contents |
|---|---|
| `main` | this exercise workspace |
| `my_first_project` | the first real project (its own crate, README and learning pack) |

## Conventions

- One exercise = one file in `src/bin/`, named `exNN.rs`.
- Fix every warning before ticking an exercise off.
- Commit per exercise: `git commit -m "ex01: print & math"`.
- Keep `target/` out of git (already in `.gitignore`).

## Resources

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [std docs](https://doc.rust-lang.org/std/)
