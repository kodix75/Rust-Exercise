# Exercises 01–03 — Language basics

> Functions, mutability and loops. These three are pure logic: no input, no files.
>
> Run one: `npm run ex -- ex01` · Test one: `npm run test:ex -- ex01`

**In this file:** Ex 01 · Ex 02 · Ex 03

---

## Ex 01 — Print & math

> **Concepts:** functions · parameters · return values · `f64` · `println!`
> **File:** `src/bin/ex01.rs` · **Run:** `npm run ex -- ex01`

### Goal

Write four functions — `add`, `sub`, `mul`, `div` — that each take two
`f64` and return an `f64`. In `main`, print the four results for
`a = 12.0` and `b = 4.0`.

This exercise is really about **function signatures**: name, parameters,
types, and return type.

### Starter

```rust
fn add(a: f64, b: f64) -> f64 {
    a + b
}

fn sub(a: f64, b: f64) -> f64 {
    a - b
}

fn mul(a: f64, b: f64) -> f64 {
    a * b
}

fn div(a: f64, b: f64) -> f64 {
    a / b
}

fn main() {
    let a = 12.0;
    let b = 4.0;

    println!("sum      = {}", add(a, b));
    println!("diff     = {}", sub(a, b));
    println!("product  = {}", mul(a, b));
    println!("quotient = {}", div(a, b));
}
```

### Expected output

```text
sum      = 16
diff     = 8
product  = 48
quotient = 3
```

### Hints

<details>
<summary>Hint — why <code>12.0</code> and not <code>12</code>?</summary>

`12` is an integer, `12.0` is a float. The functions expect `f64`, so use
floats everywhere. Mixing them gives a `mismatched types` error.
</details>

<details>
<summary>Hint — the alignment</summary>

The spaces after `sum`, `diff`, … are there to line the `=` up. They are
part of the string, not something Rust does for you.
</details>

### Read & search

- [Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html) — parameters and return values
- [Scalar types](https://doc.rust-lang.org/book/ch03-02-data-types.html) — `f64` vs integers
- [`println!` formatting](https://doc.rust-lang.org/std/fmt/) — `{}` and friends

### Stretch

- Add a `modulo(a, b) -> f64` that uses `%` (works on floats in Rust).
- Make `div` return `Option<f64>`: `None` when `b` is zero.

---

## Ex 02 — Mutability & shadowing

> **Concepts:** `let` · `mut` · shadowing · reading compiler errors
> **File:** `src/bin/ex02.rs` · **Run:** `npm run ex -- ex02`

### Goal

Learn the difference between **changing a value** and **creating a new
binding with the same name**.

1. Start with `let n = 10;`.
2. Try `n += 1;`. Read the error before changing anything.
3. Fix it with `let mut n = 10;`.
4. Then shadow it: `let n = n * 2;` and print the result.

### Starter

```rust
fn main() {
    let n = 10;

    // 1. Uncomment the next line and run it. Read the error.
    // n += 1;

    // 2. Then make `n` mutable and run again.

    // 3. Shadowing: this creates a NEW variable, it does not mutate the old one.
    let n = n * 2;

    println!("n = {n}");
}
```

### Expected output

```text
n = 20
```

(After step 2 the value is 11; after shadowing it becomes 22. Adjust the
final print to see each stage.)

### Hints

<details>
<summary>Hint — what the error means</summary>

`cannot assign to n, as it is not declared as mutable` means the binding is
read-only. Add `mut` where the variable is created, not where it is used.
</details>

<details>
<summary>Hint — mutation vs shadowing</summary>

```rust
let mut a = 10;
a += 1;          // mutation: same variable, new value

let b = 10;
let b = b + 1;   // shadowing: a brand-new variable named b
```

Shadowing can even change the type: `let x = 5; let x = "five";`.
</details>

### Read & search

- [Variables and mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
- [Shadowing](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing)

### Stretch

- Shadow `n` as a `String` (`let n = n.to_string();`) — it compiles.
- Try to use the old `n` after a move and read the error.

---

## Ex 03 — FizzBuzz

> **Concepts:** `for` · ranges · `if` / `else if` · modulo `%`
> **File:** `src/bin/ex03.rs` · **Run:** `npm run ex -- ex03`

### Goal

Print the numbers 1 to 30. But:

- multiples of **3** print `Fizz`
- multiples of **5** print `Buzz`
- multiples of **both** print `FizzBuzz`
- everything else prints the number

The classic interview question — and a perfect `for` loop drill.

### Starter

```rust
fn main() {
    for n in 1..=30 {
        // your logic here
        println!("{n}");
    }
}
```

### Expected output (first lines)

```text
1
2
Fizz
4
Buzz
Fizz
7
8
Fizz
Buzz
11
Fizz
13
14
FizzBuzz
16
...
```

### Hints

<details>
<summary>Hint — order of the checks</summary>

Check **both** conditions first. If you test `n % 3 == 0` first, 15 prints
`Fizz` and never reaches `FizzBuzz`.

```rust
if n % 3 == 0 && n % 5 == 0 {
    println!("FizzBuzz");
} else if n % 3 == 0 {
    println!("Fizz");
} else if n % 5 == 0 {
    println!("Buzz");
} else {
    println!("{n}");
}
```
</details>

<details>
<summary>Hint — the shorter version</summary>

`n % 15 == 0` is exactly the same as "divisible by 3 **and** 5", because 3
and 5 are coprime.
</details>

### Read & search

- [`for` loops and ranges](https://doc.rust-lang.org/book/ch03-05-control-flow.html#looping-through-a-collection-with-for)
- [Arithmetic operators](https://doc.rust-lang.org/book/appendix-02-operators.html)

### Stretch

- Put the logic in `fn fizzbuzz(n: u32) -> String` and print the result in
  `main`. Now it is testable:

  ```rust
  #[test]
  fn fifteen_is_fizzbuzz() {
      assert_eq!(fizzbuzz(15), "FizzBuzz");
  }
  ```

---
