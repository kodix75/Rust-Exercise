# Exercises 04–06 — Loops, collections and input

> Recursion, iterators, and the first program that talks to you.
>
> Run one: `npm run ex -- ex04` · Test one: `npm run test:ex -- ex04`

**In this file:** Ex 04 · Ex 05 · Ex 06

---

## Ex 04 — Factorial

> **Concepts:** `while` / `for` · recursion · expressions as return values
> **File:** `src/bin/ex04.rs` · **Run:** `npm run ex -- ex04`

### Goal

Write factorial **twice**:

```rust
fn factorial_iter(n: u64) -> u64   // with a loop
fn factorial_rec(n: u64) -> u64    // with recursion
```

Print both for `n = 5` and `n = 10`. They must agree.

`5! = 5 × 4 × 3 × 2 × 1 = 120`

### Starter

```rust
fn factorial_iter(n: u64) -> u64 {
    let mut result = 1;
    // multiply result by every number from 1 to n
    result
}

fn factorial_rec(n: u64) -> u64 {
    // base case, then n * factorial_rec(n - 1)
    1
}

fn main() {
    for n in [5, 10] {
        println!(
            "{n}! = {} (iter) / {} (rec)",
            factorial_iter(n),
            factorial_rec(n)
        );
    }
}
```

### Expected output

```text
5! = 120 (iter) / 120 (rec)
10! = 3628800 (iter) / 3628800 (rec)
```

### Hints

<details>
<summary>Hint — the loop</summary>

```rust
let mut result = 1;
for i in 1..=n {
    result *= i;
}
result
```
</details>

<details>
<summary>Hint — the recursion</summary>

Every recursion needs a **base case**, otherwise it never stops:

```rust
if n <= 1 {
    1
} else {
    n * factorial_rec(n - 1)
}
```

The last expression has no semicolon — that is the return value.
</details>

<details>
<summary>Hint — why <code>u64</code>?</summary>

Factorials explode fast. `u64` overflows at `21!`. Try `20!` and then `21!`
in debug mode: Rust panics on overflow instead of silently wrapping.
</details>

### Read & search

- [Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
- [Recursion example](https://doc.rust-lang.org/rust-by-example/fn/recursion.html)

### Stretch

- Return `Option<u64>` and use `checked_mul` so big inputs return `None`
  instead of panicking.
- Write a `#[test]` that checks `factorial_iter(10) == factorial_rec(10)`.

---

## Ex 05 — Vec statistics

> **Concepts:** `Vec` · iterators · `sum` · `max` · `sort` · `filter`
> **File:** `src/bin/ex05.rs` · **Run:** `npm run ex -- ex05`

### Goal

Given:

```rust
let v = vec![5, 2, 9, 1, 7, 2];
```

compute and print:

- the **mean** (average)
- the **minimum**
- the **maximum**
- a **sorted copy**
- how many times the value `2` appears

### Starter

```rust
fn main() {
    let v = vec![5, 2, 9, 1, 7, 2];

    let sum: i32 = v.iter().sum();
    let mean = sum as f64 / v.len() as f64;

    println!("values = {v:?}");
    println!("mean   = {mean:.2}");
    // min, max, sorted, count of 2
}
```

### Expected output

```text
values = [5, 2, 9, 1, 7, 2]
mean   = 4.33
min    = 1
max    = 9
sorted = [1, 2, 2, 5, 7, 9]
count  = 2
```

### Hints

<details>
<summary>Hint — min / max</summary>

```rust
let min = v.iter().min().unwrap();
let max = v.iter().max().unwrap();
```

`min()` returns `Option<&i32>` because the vector could be empty — hence
`unwrap()`. (Later: handle `None` properly.)
</details>

<details>
<summary>Hint — sorted copy</summary>

`v.sort()` changes `v` in place, and needs `let mut v`. For a **copy**:

```rust
let mut sorted = v.clone();
sorted.sort();
```

Or with iterators: `let mut sorted: Vec<i32> = v.to_vec();`
</details>

<details>
<summary>Hint — count</summary>

```rust
let twos = v.iter().filter(|&&x| x == 2).count();
```

`filter` gives you `&&i32` here (a reference to a reference), which is why
the pattern is `|&&x|`. `v.iter().filter(|x| **x == 2)` is the same thing.
</details>

### Read & search

- [`Vec`](https://doc.rust-lang.org/book/ch08-01-vectors.html)
- [Iterator methods](https://doc.rust-lang.org/std/iter/trait.Iterator.html) — search for `sum`, `min`, `max`, `filter`, `count`

### Stretch

- Print the **median** (middle value of the sorted vector).
- Use `fold` instead of `sum` to compute the mean in one pass.

---

## Ex 06 — Guess the number ⭐

> **Concepts:** `loop` · `stdin` · `parse` · `Result` · `match`
> **File:** `src/bin/ex06.rs` · **Run:** `npm run ex -- ex06`

### Goal

The first **interactive** program.

- The secret number is hardcoded (e.g. `42`), between 1 and 100.
- Loop: read a line from the keyboard.
- Parse it as a number — that can fail, so handle the `Result`.
- Print `too high` / `too low` / `win`, and count the attempts.

### Starter

```rust
use std::io;

fn main() {
    let secret = 42;
    let mut attempts = 0;

    println!("Guess the number (1-100)");

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");

        let guess: u32 = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("that is not a number, try again");
                continue;
            }
        };

        attempts += 1;

        // compare guess with secret
    }
}
```

### Example session

```text
Guess the number (1-100)
50
too high
25
too low
42
win in 3 attempts
```

### Hints

<details>
<summary>Hint — the three cases</summary>

```rust
if guess > secret {
    println!("too high");
} else if guess < secret {
    println!("too low");
} else {
    println!("win in {attempts} attempts");
    break;
}
```
</details>

<details>
<summary>Hint — why <code>trim()</code>?</summary>

`read_line` keeps the newline you typed. `"42\n"` does not parse; `"42"`
does. `trim()` removes it — and any accidental spaces.
</details>

<details>
<summary>Hint — why <code>match</code> instead of <code>unwrap</code>?</summary>

`unwrap()` panics on bad input, which ends the program. Here we want to say
"not a number" and keep looping, so we match on `Ok` / `Err` explicitly.
</details>

### Read & search

- [Handling user input](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html) — the Book builds this exact program
- [`Result`](https://doc.rust-lang.org/std/result/)
- [`str::parse`](https://doc.rust-lang.org/std/primitive.str.html#method.parse)

### Stretch

- Use the [`rand`](https://crates.io/crates/rand) crate for a real random
  secret: `Cargo.toml` → `rand = "0.9"`, then `rand::rng().random_range(1..=100)`.
- Reject numbers outside 1..=100 without counting an attempt.
- Give a hint after 5 failed attempts (`the number is even`, …).

---
