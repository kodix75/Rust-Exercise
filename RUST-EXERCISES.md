# Rust Exercises — Complete Learning Pack

> **How to use this file**
> 1. Create exercises in `src/bin/exNN.rs` (one file per exercise).
> 2. Run: `cargo run --bin exNN`
> 3. Check: `cargo check --bin exNN` — fix **every** warning.
> 4. Tick the box `☐ → ☑` when it runs cleanly.
> 5. Commit each one: `git add . && git commit -m "exNN: ..."`

---

## 📑 Table of Contents

1. [Setup](#1-setup)
2. [Where the code goes](#2-where-the-code-goes)
3. [Syntax cheat-sheet](#3-syntax-cheat-sheet)
4. [Exercises 1–10](#4-exercises)
5. [First real project ideas](#5-first-real-project-ideas)
6. [Roadmap: now vs later](#6-roadmap-now-vs-later)
7. [Common errors](#7-common-errors)
8. [Commands reference](#8-commands-reference)
9. [Fedora / system-Rust notes](#9-fedora--system-rust-notes)
10. [Learning Log](#10-learning-log)

---

## 1. Setup

### 1.1 Install Rust (Fedora)

```bash
sudo dnf install rust cargo
rustc --version
cargo --version
```

> If you already have `rustc 1.99.1 (… Fedora …)`, you're set.

### 1.2 Install rust-analyzer support (Fedora-specific fix)

Your `rustc` is Fedora's **system Rust** with sysroot `/usr`. rust-analyzer needs the standard-library **source**, which is a separate package:

```bash
sudo dnf install rust-src
```

Verify:
```bash
ls /usr/lib/rustlib/src/rust/library/core/src/lib.rs
```

If you see `No such file`, reinstall: `sudo dnf reinstall rust-src`.

### 1.3 Check for stray `RUST_SRC_PATH`

```bash
echo $RUST_SRC_PATH
```

If it prints **anything**, unset it (`unset RUST_SRC_PATH`) and remove the `export …` line from `~/.bashrc` / `~/.zshrc`. A wrong `RUST_SRC_PATH` overrides the sysroot and breaks rust-analyzer.

### 1.4 VS Code setup

1. Install extension **rust-analyzer**.
2. Open the project folder (`File → Open Folder` → select `my_first_project`).
3. If you see "sysroot errors", **restart the server**:
   `Ctrl+Shift+P` → *Rust Analyzer: Restart server*.

### 1.5 If the sysroot error persists

Pin the sysroot manually:
1. `rustc --print sysroot` → on Fedora prints `/usr`.
2. VS Code → `Ctrl+,` → search `rust-analyzer.cargo.sysroot`.
3. Set it to `/usr` (exact output).
4. Restart rust-analyzer server.

### 1.6 Install clippy (linter)

```bash
sudo dnf install clippy
```

Run: `cargo clippy --bin exNN` — it teaches you idiomatic Rust for free.

### 1.7 Create a project

```bash
cargo new my_first_project
cd my_first_project
```

**Layout created:**
```
my_first_project/
├── Cargo.toml      ← project metadata + dependencies
├── Cargo.lock      ← auto, never edit
├── .gitignore      ← auto
└── src/
    └── main.rs     ← entry point
```

---

## 2. Where the code goes

### Rule #1: **All Rust code lives inside `src/`. Nothing else.**

### Layout you should build

```
my_first_project/
├── Cargo.toml
├── Cargo.lock
├── RUST-EXERCISES.md      ← this file
└── src/
    ├── main.rs            ← final project (leave hello-world for now)
    └── bin/               ← CREATE THIS
        ├── ex01.rs        ← one file = one runnable program
        ├── ex02.rs
        ├── ex03.rs
        └── …
```

### Create the `bin/` folder

```bash
mkdir src/bin
```

### Run an exercise

```bash
cargo run --bin ex01
cargo run --bin ex03
```

### Structure of any `.rs` file

```rust
// imports
use std::io;

fn main() {
    // code that RUNS goes here
}

// everything else at top level:
fn helper(a: i32, b: i32) -> i32 { a + b }
struct Point { x: f64, y: f64 }
impl Point { fn new() -> Point { Point { x: 0.0, y: 0.0 } } }
enum Shape { Circle(f64) }
```

**Key rules:**
- 4 spaces indentation (Rust convention).
- `fn main()` is required for a *binary*; helpers live **outside** main, at top level.
- Never create `.rs.txt` — file must end in `.rs`.

---

## 3. Syntax cheat-sheet

```rust
fn main() {
    // --- variables ---
    let x = 5;                 // immutable
    let mut y = 10;            // mutable
    y += 1;
    let x = "shadowed";        // shadowing is allowed

    // --- types ---
    let a: i32 = -42;          // i8 i16 i32 i64 i128 isize
    let b: u64 = 42;           // unsigned
    let c: f64 = 3.14;
    let d: bool = true;
    let e: char = 'é';
    let s: &str = "borrowed";
    let owned: String = String::from("owned");

    // --- printing ---
    println!("x = {}, name = {}", x, s);
    println!("{c:.2}");                 // 3.14
    eprintln!("goes to stderr");

    // --- conditions ---
    if x as i32 > 3 { println!("big"); } else { println!("small"); }
    let label = if true { "yes" } else { "no" };   // if is an expression

    // --- loops ---
    for i in 0..5   { println!("{i}"); }   // exclusive
    for i in 0..=5  { }                    // inclusive
    let mut n = 0;
    while n < 3 { n += 1; }
    loop { break; }                        // loop can return a value

    // --- collections ---
    let arr = [1, 2, 3];                   // fixed size
    let tup = (1, "two", 3.0);             // tup.0, tup.1
    let mut v: Vec<i32> = vec![1, 2, 3];
    v.push(4);

    // --- functions ---
    fn add(a: i32, b: i32) -> i32 { a + b }
    println!("{}", add(2, 3));

    // --- struct + impl ---
    struct Point { x: f64, y: f64 }
    impl Point {
        fn dist_to_origin(&self) -> f64 {
            (self.x * self.x + self.y * self.y).sqrt()
        }
    }
    let p = Point { x: 3.0, y: 4.0 };
    println!("{}", p.dist_to_origin());    // 5

    // --- enums + match ---
    enum Shape { Circle(f64), Square(f64) }
    fn area(s: &Shape) -> f64 {
        match s {
            Shape::Circle(r) => 3.14159 * r * r,
            Shape::Square(a) => a * a,
        }
    }

    // --- Option / Result ---
    fn divide(a: f64, b: f64) -> Option<f64> {
        if b == 0.0 { None } else { Some(a / b) }
    }
    match divide(1.0, 0.0) {
        Some(v) => println!("{v}"),
        None    => println!("cannot divide by zero"),
    }

    // --- ownership ---
    let s1 = String::from("hi");
    let s2 = s1;                     // s1 is MOVED
    let s3 = s2.clone();             // deep copy
    fn len(s: &String) -> usize { s.len() }   // borrow with &
    println!("{}", len(&s2));
}
```

---

## 4. Exercises

### ☐ Ex 1 — Print & math

**File:** `src/bin/ex01.rs`

Four functions: `add`, `sub`, `mul`, `div`. Each takes two `f64`, returns `f64`.
In `main`, print each result for `a = 12.0`, `b = 4.0`.

**Expected output:**
```
sum      = 16
diff     = 8
product  = 48
quotient = 3
```

**Starter:**
```rust
fn add(a: f64, b: f64) -> f64 { a + b }
fn sub(a: f64, b: f64) -> f64 { a - b }
fn mul(a: f64, b: f64) -> f64 { a * b }
fn div(a: f64, b: f64) -> f64 { a / b }

fn main() {
    let a = 12.0;
    let b = 4.0;
    println!("sum      = {}", add(a, b));
    println!("diff     = {}", sub(a, b));
    println!("product  = {}", mul(a, b));
    println!("quotient = {}", div(a, b));
}
```

**Practice:** `fn`, parameters, return values, `println!`.

---

### ☐ Ex 2 — Mutability & shadowing

**File:** `src/bin/ex02.rs`

1. `let n = 10;`
2. Try `n += 1;` — **read the compiler error**.
3. Fix with `let mut n = 10;`.
4. Re-declare: `let n = n * 2;` (shadowing).

**Practice:** `mut`, shadowing, reading compiler errors.

---

### ☐ Ex 3 — FizzBuzz

**File:** `src/bin/ex03.rs`

Print 1 to 30.
- Multiple of 3 → `Fizz`
- Multiple of 5 → `Buzz`
- Both → `FizzBuzz`
- Otherwise → the number

**Practice:** `for`, `if`, `%`.

---

### ☐ Ex 4 — Factorial

**File:** `src/bin/ex04.rs`

`fn factorial_iter(n: u64) -> u64` (loop) and `fn factorial_rec(n: u64) -> u64` (recursion).
Print both for `n = 5` and `n = 10`.

**Practice:** loops, recursion, expressions as return values.

---

### ☐ Ex 5 — Vec statistics

**File:** `src/bin/ex05.rs`

Given `let v = vec![5, 2, 9, 1, 7, 2];`, compute and print:
- mean
- min
- max
- sorted copy
- count of the value `2`

**Practice:** `Vec`, iterators (`.iter()`, `.sum()`, `.max()`, `.sort()`).

---

### ☐ Ex 6 — Guess the number ⭐

**File:** `src/bin/ex06.rs`

- Secret number hardcoded 1..=100 (later: use `rand` crate).
- Loop: read input with `std::io::stdin().read_line(&mut input)`.
- Parse with `input.trim().parse::<u32>()` (returns `Result`).
- Print “too high” / “too low” / “win”, count attempts.

**Practice:** `loop`, `match`, `Result`, string handling.

---

### ☐ Ex 7 — BankAccount struct

**File:** `src/bin/ex07.rs`

```rust
struct BankAccount {
    owner: String,
    balance: f64,
}
```

Methods:
- `deposit(&mut self, amount: f64)`
- `withdraw(&mut self, amount: f64) -> Result<(), String>` (error if insufficient)
- `display(&self)`

**Practice:** `struct`, `impl`, `&mut self`, `Result`.

---

### ☐ Ex 8 — Enum calculator

**File:** `src/bin/ex08.rs`

```rust
enum Op { Add, Sub, Mul, Div }

fn apply(op: Op, a: f64, b: f64) -> Result<f64, String>
```

Division by zero returns `Err("cannot divide by zero")`.
Call all four ops and handle the `Result`.

**Practice:** `enum`, `match`, error handling.

---

### ☐ Ex 9 — Read a file

**File:** `src/bin/ex09.rs`

Read a text file, print:
- number of lines
- number of words
- number of characters

Use `std::fs::read_to_string` and `?` inside:
```rust
fn main() -> Result<(), Box<dyn std::error::Error>>
```

**Practice:** `Result`, `?`, file I/O.

---

### ☐ Ex 10 — CLI arguments

**File:** `src/bin/ex10.rs`

Read `std::env::args()`, print them.
Then implement: `cargo run --bin ex10 -- add 3 4` → prints `7`.

> Note the `--` between the binary name and the program's args.

**Practice:** `Vec<String>`, parsing, argument validation.

---

## 5. First real project ideas

Pick **one** after Ex 6 (or Ex 9 if you like the CLI path).

| # | Project | What you practice | Difficulty |
|---|---------|-------------------|------------|
| 1 | **Unit converter** (`conv 5 km mi`) | args, parse, match, functions | ⭐ |
| 2 | **Password generator** (length + chars) | `rand`, `Vec<char>`, args | ⭐ |
| 3 | **Word / char counter from a file** | fs, strings, iterators | ⭐⭐ |
| 4 | **TODO list CLI with file persistence** ⭐ recommended | structs, enums, fs, args, `Result` | ⭐⭐ |
| 5 | **Quiz game** (questions from a file, score) | fs, structs, loops, input | ⭐⭐ |
| 6 | **Expense tracker** (CSV → totals per category) | parsing, `HashMap`, sorting | ⭐⭐⭐ |

### Why #4 TODO CLI

It exercises **everything** you need for real Rust: structs, enums (commands), `Result`/`?`, file I/O, argument parsing, `Vec`, `match`. It's also genuinely useful.

**Sketch:**
```rust
enum Command { Add(String), List, Done(usize), Remove(usize) }

struct Task { title: String, done: bool }

fn load(path: &str) -> Result<Vec<Task>, std::io::Error> { /* … */ }
fn save(path: &str, tasks: &[Task]) -> Result<(), std::io::Error> { /* … */ }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cmd = parse_args()?;
    let mut tasks = load("tasks.txt")?;
    // match cmd { … }
    save("tasks.txt", &tasks)?;
    Ok(())
}
```

When you start it, put the code in **`src/main.rs`**, not in `src/bin/`.
- `cargo run` → your real program
- `cargo run --bin exNN` → exercises

---

## 6. Roadmap: now vs later

### Learn now (needed for the exercises and first project)
- variables, `mut`, shadowing
- types (`i32`, `f64`, `bool`, `char`, `String`, `&str`)
- `if` / `loop` / `for` / `while`
- functions, return values
- `Vec`, tuples, arrays
- structs + `impl`
- enums + `match`
- `Option` / `Result` + `?`
- borrowing basics (`&`, `&mut`)
- `cargo new / run / check / build`
- adding a crate to `Cargo.toml`

### Learn later (after first project works)
- lifetimes
- traits + generics
- iterators in depth
- modules (`mod`, multiple files)
- tests (`#[test]`)
- `HashMap`
- closures
- error types (`thiserror`)
- `serde` for JSON

### Do NOT dive into yet
- `async`/`tokio`
- writing macros
- `unsafe`
- advanced lifetime signatures (`'a` in fn params)
- trait objects (`dyn Trait`) beyond `Box<dyn Error>`
- GUI, WASM, embedded

---

## 7. Common errors

| Error | Cause | Fix |
|-------|-------|-----|
| `could not find Cargo.toml` | Ran `cargo` outside project root | `cd my_first_project` |
| `no bin target named ex01` | File not in `src/bin/` or wrong name (e.g. `ex01.rs.txt`) | Path must be `src/bin/ex01.rs` |
| `cannot find function 'x'` | Typo, or `fn x` defined inside `main` | Define `fn x` at top level (outside `main`) |
| `expected expression, found '}'` | Missing value / mismatched braces | Read the error line — it's exact |
| `cannot assign to 'x', as it is not declared as mutable` | `x = …` on `let x` (no `mut`) | `let mut x = …` |
| `warning: unused variable: 'a'` | Declared, not used | Use it, prefix `_a`, or delete |
| `use of moved value` | `let b = a;` then used `a` | `.clone()` or borrow `&a` |
| `mismatched types` | `i32` vs `f64` etc. | Cast: `x as f64`, or `.parse::<f64>()` |
| `cannot find value 'x' in this scope` | Used before declaring | Move `let x` above |

**Golden rule:** always fix the **first** error. Later errors are often cascades.

---

## 8. Commands reference

```bash
cargo new NAME              # create project
cargo run                   # run src/main.rs
cargo run --bin ex01        # run src/bin/ex01.rs
cargo run --bin ex10 -- add 3 4   # run with args (note the --)
cargo check                 # fast type-check everything
cargo check --bin ex01      # check just one
cargo build                 # produce binary in target/debug/
cargo build --release       # optimized binary
cargo fmt                   # auto-format all code
cargo clippy --bin ex01     # lint with suggestions
cargo clean                 # delete target/
```

### Git (already initialized by `cargo new`)
```bash
git add .
git commit -m "ex01: done"
git log --oneline
```

---

## 9. Fedora / system-Rust notes

Your setup: `rustc` from `dnf` (Fedora package), sysroot = `/usr`.

| Symptom | Fix |
|---------|-----|
| rust-analyzer: `can't load standard library from sysroot /usr` | `sudo dnf install rust-src`, then restart rust-analyzer |
| Still broken after install | `sudo dnf reinstall rust-src` |
| Still broken | Check `echo $RUST_SRC_PATH` — unset if non-empty |
| Still broken | VS Code setting `rust-analyzer.cargo.sysroot` = `/usr` |
| Want rustup instead (optional) | Install rustup, but then remove system `rust`: two toolchains can confuse rust-analyzer |

**Restart rust-analyzer after every fix:**
`Ctrl+Shift+P` → *Rust Analyzer: Restart server*

---

## 10. Learning Log

> This section gets updated as exercises are completed.

```markdown
# Rust Learning Log

## Status
- [x] Setup done (cargo new / run / check)
- [x] rust-analyzer sysroot fixed (Fedora rust-src)
- [ ] Read syntax cheat-sheet
- [ ] Ex 1 – print & math
- [ ] Ex 2 – mutability & shadowing
- [ ] Ex 3 – FizzBuzz
- [ ] Ex 4 – factorial
- [ ] Ex 5 – Vec statistics
- [ ] Ex 6 – Guess the number ⭐
- [ ] Ex 7 – BankAccount struct
- [ ] Ex 8 – Enum calculator
- [ ] Ex 9 – Read a file
- [ ] Ex 10 – CLI args
- [ ] First project chosen: ___________
- [ ] First project working
- [ ] First project polished (errors, README)

## Concepts covered
- (none yet — starting point)

## Concepts to revisit
- (nothing yet)

## Explicitly deferred
async/tokio, macros, unsafe, advanced lifetimes,
traits & generics in depth, GUI, WASM, embedded

## Environment
- OS: Fedora 44
- Rust: 1.99.1 (Fedora package, sysroot /usr)
- rust-analyzer: VS Code extension
- Project path: /home/aidan/Projects/Github/my_first_project
- Next action: write src/bin/ex01.rs and run `cargo run --bin ex01`
```

---

## ✅ Next steps right now

1. Confirm rust-analyzer reports no sysroot error.
2. Create `src/bin/ex01.rs`, paste the Ex 1 starter, save.
3. Run: `cargo run --bin ex01`
4. Fix every warning (`cargo check --bin ex01`).
5. Commit: `git add . && git commit -m "ex01: print & math"`
6. Tick Ex 1 in the Learning Log, then move to Ex 2.
