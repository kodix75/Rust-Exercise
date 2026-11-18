<div align="center">

# 🦀 Rust Exercises

**Ten small programs, one per concept — built to be run, broken and understood.**

[![CI](https://github.com/kodix75/Rust-Exercise/actions/workflows/ci.yml/badge.svg)](https://github.com/kodix75/Rust-Exercise/actions/workflows/ci.yml)
[![Rust](https://img.shields.io/badge/rust-1.99-000?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![npm](https://img.shields.io/badge/run%20with-npm-cb3837?logo=npm&logoColor=white)](package.json)
[![License: MIT](https://img.shields.io/badge/license-MIT-2ea44f)](LICENSE)
[![PRs welcome](https://img.shields.io/badge/PRs-welcome-8957e5)](../../pulls)

[Quick start](#-quick-start) · [Exercises](#-the-ten-exercises) · [Syntax sheet](docs/SYNTAX.md) · [How to practise](#-the-workflow) · [Project branch](https://github.com/kodix75/Rust-Exercise/tree/my_first_project)

</div>

---

## Why this repo

Reading Rust is easy. **Writing** Rust is a different skill, and it only
develops by compiling your own code and fixing your own errors.

So this repo is not a library and not a tutorial: it is a **workbench**.
Each exercise is one tiny binary that you write from scratch, run, break and
repair. The explanations are here so you never get stuck for more than a
few minutes; the code is yours to write.

> **The rule:** no copy-paste without reading. If it does not compile, the
> compiler is teaching you something — read the first error, fix it, repeat.

---

## ⚡ Quick start

```bash
git clone git@github.com:kodix75/Rust-Exercise.git
cd Rust-Exercise

npm run ex -- ex01     # run exercise 1
npm test               # verify the whole workspace
```

That is it. `npm` is only the **remote control**; `cargo` and `rustc` do the
actual compiling underneath.

### Requirements

| Tool | Version | Install (Fedora) |
|---|---|---|
| Rust + cargo | 1.99 | `sudo dnf install rust cargo` |
| rust-src (rust-analyzer) | — | `sudo dnf install rust-src` |
| rustfmt | — | `sudo dnf install rustfmt` |
| clippy | — | `sudo dnf install clippy` |
| Node + npm | ≥ 20 | `sudo dnf install nodejs npm` |

`npm test` skips `fmt` and `clippy` automatically when they are missing, so a
minimal install still works.

---

## 🎛 npm commands

npm cannot compile Rust — `cargo` does that. What `package.json` gives you is
**one memorable command set** for the whole workflow, the same one you already
use in your JavaScript repos.

| Command | Does | Under the hood |
|---|---|---|
| `npm run ex -- ex01` | run one exercise | `cargo run --bin ex01` |
| `npm run ex -- ex10 -- add 3 4` | run it with arguments | `cargo run --bin ex10 -- add 3 4` |
| `npm run test:ex -- ex01` | **check one exercise** | `cargo test --bin ex01 -- --ignored` |
| `npm run test:all` | check all ten exercises | same, one after another |
| `npm run check` | type-check everything | `cargo check --all-targets` |
| `npm run build` | compile | `cargo build --all-targets` |
| `npm test` | **repository check** | structure + `cargo test` + fmt + clippy |
| `npm run test:rust` | Rust tests only | `cargo test` |
| `npm run lint` | lints, warnings are errors | `cargo clippy -- -D warnings` |
| `npm run fmt` | check formatting | `cargo fmt -- --check` |
| `npm run fmt:fix` | reformat the code | `cargo fmt` |

`npm run ex` with no argument lists the exercises that exist. If you prefer
raw cargo, every command above is shown in the right-hand column — nothing is
hidden behind a wrapper.

---

## 🗂 Repository layout

```text
Rust-Exercise/
├── src/
│   ├── main.rs              # placeholder — the exercises live in bin/
│   └── bin/                 # one file = one runnable program
│       ├── ex01.rs          # instructions in comments + TODO + one test
│       └── … ex10.rs
├── docs/
│   ├── SYNTAX.md            # one-page Rust reference
│   └── exercises/           # guides grouped 2-3 exercises per file
├── tests/
│   └── conventions.rs       # repo rules, checked by cargo test
├── scripts/
│   ├── exercise.mjs         # npm run ex
│   ├── test-exercise.mjs    # npm run test:ex
│   └── verify.mjs           # npm test
├── .github/workflows/ci.yml # CI on every push
├── package.json             # npm command layer
├── Cargo.toml               # the real build config
└── rustfmt.toml             # formatting rules
```

**All Rust code lives in `src/`.** One exercise = one file in `src/bin/`,
named `exNN.rs`. Each file is a complete program with its own `fn main()`.

---

## 🎯 The ten exercises

| # | Exercise | You learn | Guide |
|:--:|---|---|:--:|
| 01 | **Print & math** | functions, `f64`, `println!` | [→](docs/exercises/01-03-basics.md) |
| 02 | **Mutability & shadowing** | `let`, `mut`, reading errors | [→](docs/exercises/01-03-basics.md) |
| 03 | **FizzBuzz** | `for`, ranges, `%` | [→](docs/exercises/01-03-basics.md) |
| 04 | **Factorial** | loops vs recursion | [→](docs/exercises/04-06-loops-and-input.md) |
| 05 | **Vec statistics** | `Vec`, iterators | [→](docs/exercises/04-06-loops-and-input.md) |
| 06 | **Guess the number** ⭐ | input, `Result`, `match` | [→](docs/exercises/04-06-loops-and-input.md) |
| 07 | **BankAccount** | `struct`, `impl`, `&mut self` | [→](docs/exercises/07-08-structs-and-enums.md) |
| 08 | **Enum calculator** | `enum`, exhaustive `match` | [→](docs/exercises/07-08-structs-and-enums.md) |
| 09 | **Read a file** | `fs`, `?`, `Result` in `main` | [→](docs/exercises/09-10-files-and-args.md) |
| 10 | **CLI arguments** | `env::args`, validation | [→](docs/exercises/09-10-files-and-args.md) |

Every `src/bin/exNN.rs` **already exists**, with the instructions in a
comment at the top, a runnable `main`, a `TODO` where your code goes, and
**one test** at the bottom. The guides are grouped two or three per file so
a whole topic reads in one go.

Each guide has the same shape:

```text
goal → starter code → expected output → collapsible hints → pages to read → stretch goals
```

Hints are folded away on purpose. Try first, unfold second.

---

## 🔁 The workflow

For every exercise:

1. **Open** `src/bin/exNN.rs` and read the comment at the top.
2. **Read** the matching guide in `docs/exercises/` if you need more detail.
3. **Fill in** the `TODO`s — the starter code compiles, but does nothing yet.
4. **Run** `npm run ex -- exNN` and compare with the expected output.
5. **Test** `npm run test:ex -- exNN` — green means the exercise is correct.
6. **Pass the repo check** with `npm test`, then tick the box and commit.

```bash
git add src/bin/ex01.rs
git commit -m "ex01: print & math"
```

Each file's test is marked `#[ignore]` on purpose: that keeps `npm test` and
CI green while an exercise is still unsolved, and the runner un-ignores it
for you.

### What `npm test` checks

| Check | Fails when |
|---|---|
| **structure** | a file in `src/bin/` is not `exNN.rs`, or its guide is missing |
| **cargo check** | the code does not compile |
| **cargo test** | the repo conventions in `tests/conventions.rs` are broken |
| **cargo fmt** | the code is not formatted (skipped if rustfmt is missing) |
| **cargo clippy** | there is any warning (skipped if clippy is missing) |

CI runs the same command on every push, so a green badge means the workspace
is genuinely clean.

---

## 📖 Syntax reference

[`docs/SYNTAX.md`](docs/SYNTAX.md) is the one-page cheat-sheet used by these
exercises: variables, types, printing, conditions, loops, collections,
functions, structs, enums, `Option`/`Result`, ownership and borrowing — plus
the compiler errors you will meet most often.

The full curriculum (setup, explanations, project ideas, learning log) is on
the [**`my_first_project`**](https://github.com/kodix75/Rust-Exercise/tree/my_first_project)
branch, in [`RUST-EXERCISES.md`](https://github.com/kodix75/Rust-Exercise/blob/my_first_project/RUST-EXERCISES.md).

---

## 🌿 Branches

| Branch | Contents | Local checkout |
|---|---|---|
| `main` | this exercise workspace | `Rust-Exercise/` |
| `my_first_project` | the first real project, its own crate + full learning pack | `~/Projects/Github/my_first_project` (git worktree) |

The second branch is checked out as a **worktree**, so both branches can be
open at the same time without copying the repository.

---

## 🗺 Roadmap

- [x] Workspace scaffold + npm command layer
- [x] Ten exercise files with instructions in comments and one test each
- [x] Guides grouped 2–3 per file, with hints and expected output
- [x] Syntax cheat-sheet
- [x] Conventions tests + CI
- [ ] All ten exercise tests passing
- [ ] First real project on `my_first_project`
- [ ] Solutions on a `solutions` branch (after the exercises, not before)

---

## 🤝 Contributing

This is a personal learning repo, but corrections to the guides are welcome:
open an issue or a PR. Keep one exercise per commit and make sure `npm test`
passes.

---

<div align="center">

**MIT licensed** · see [LICENSE](LICENSE)

*Compile. Fail. Read. Fix. Repeat.* 🦀

</div>
