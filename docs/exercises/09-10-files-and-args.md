# Exercises 09–10 — Files and arguments

> Connecting your program to the outside world: reading a file and parsing the command line.
>
> Run one: `npm run ex -- ex09` · Test one: `npm run test:ex -- ex09`

**In this file:** Ex 09 · Ex 10

---

## Ex 09 — Read a file

> **Concepts:** `std::fs` · `Result` · the `?` operator · `main` returning `Result`
> **File:** `src/bin/ex09.rs` · **Run:** `npm run ex -- ex09`

### Goal

Read a text file and print:

- the number of **lines**
- the number of **words**
- the number of **characters**

Use `std::fs::read_to_string` and `?` inside a `main` that returns a `Result`.

### Starter

```rust
use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::args().nth(1).unwrap_or_else(|| "sample.txt".to_string());
    let text = fs::read_to_string(&path)?;

    let lines = text.lines().count();
    let words = text.split_whitespace().count();
    let chars = text.chars().count();

    println!("file   = {path}");
    println!("lines  = {lines}");
    println!("words  = {words}");
    println!("chars  = {chars}");
    Ok(())
}
```

Create `sample.txt` next to `Cargo.toml`:

```text
Rust is fast.
Rust is safe.
```

### Expected output

```text
file   = sample.txt
lines  = 2
words  = 6
chars  = 28
```

### Hints

<details>
<summary>Hint — what does <code>?</code> do?</summary>

`fs::read_to_string(&path)?` means: "if this is an `Err`, return it from
`main` right now; otherwise give me the `String`". Without `?` you would
write a `match` every single time.

For `?` to work, the function's return type must be a `Result` — hence
`fn main() -> Result<(), Box<dyn std::error::Error>>`.
</details>

<details>
<summary>Hint — <code>lines()</code> vs <code>split('\n')</code></summary>

`lines()` also strips `\r` on Windows files and does not produce a trailing
empty line. Prefer it.
</details>

<details>
<summary>Hint — why <code>chars()</code> and not <code>len()</code>?</summary>

`String::len()` counts **bytes**. `"é"` is 2 bytes in UTF-8. `chars().count()`
counts real characters. The sample file is pure ASCII, so both agree — try
adding an accented word to see the difference.
</details>

### Read & search

- [Reading a file](https://doc.rust-lang.org/book/ch12-02-reading-a-file.html)
- [`?` in `main`](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator)
- [`str::lines`](https://doc.rust-lang.org/std/primitive.str.html#method.lines)

### Stretch

- Print the longest word.
- Count words case-insensitively with a `HashMap<&str, usize>` and print the
  top 3.
- Do not crash when the file is missing — print a friendly message instead.

---

## Ex 10 — CLI arguments

> **Concepts:** `env::args` · `Vec<String>` · parsing · argument validation
> **File:** `src/bin/ex10.rs` · **Run:** `npm run ex -- ex10 -- add 3 4`

### Goal

Read the program arguments and turn this:

```bash
npm run ex -- ex10 -- add 3 4
```

into:

```text
7
```

Support at least `add`, `sub`, `mul`, `div`. Print a usage message when the
arguments are missing or wrong.

### Starter

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{args:?}");
}
```

Run it to see what is actually in `args` — the first element is the program
path, not your first argument.

### Expected output

```text
$ npm run ex -- ex10 -- add 3 4
7

$ npm run ex -- ex10 -- mul 6 7
42

$ npm run ex -- ex10
usage: ex10 <add|sub|mul|div> <a> <b>
```

### Hints

<details>
<summary>Hint — the <code>--</code> is not yours</summary>

`npm run ex -- ex10 -- add 3 4`

- the first `--` belongs to **npm**: everything after it goes to the script
- `ex10` is read by `scripts/exercise.mjs` and becomes `--bin ex10`
- the second `--` belongs to **cargo**: everything after it goes to *your*
  program

So your program receives exactly `["add", "3", "4"]` (plus `argv[0]`).
</details>

<details>
<summary>Hint — parsing safely</summary>

```rust
let op = &args[1];
let a: f64 = args[2].parse()?;
let b: f64 = args[3].parse()?;
```

`parse()` returns a `Result`, so either use `?` in a function returning
`Result`, or match on it and print an error.

Watch out: `args[2]` panics if the user typed too few arguments. Prefer
`args.get(2)` which returns `Option<&String>`.
</details>

<details>
<summary>Hint — a clean structure</summary>

```rust
fn run(args: &[String]) -> Result<f64, String> {
    let [op, a, b] = match args {
        [op, a, b] => [op, a, b],
        _ => return Err("usage: ex10 <add|sub|mul|div> <a> <b>".into()),
    };

    let a: f64 = a.parse().map_err(|_| format!("not a number: {a}"))?;
    let b: f64 = b.parse().map_err(|_| format!("not a number: {b}"))?;

    match op.as_str() {
        "add" => Ok(a + b),
        "sub" => Ok(a - b),
        "mul" => Ok(a * b),
        "div" if b != 0.0 => Ok(a / b),
        "div" => Err("cannot divide by zero".into()),
        other => Err(format!("unknown operation: {other}")),
    }
}
```
</details>

### Read & search

- [`std::env::args`](https://doc.rust-lang.org/std/env/fn.args.html)
- [Slice patterns](https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#slice-patterns)
- [`Result::map_err`](https://doc.rust-lang.org/std/result/enum.Result.html#method.map_err)

### Stretch

- Support `pow` and `mod`.
- Accept `--help` and print the usage to stdout, exiting with code 0.
- Print invalid input to **stderr** and exit with a non-zero code:
  `std::process::exit(1)`.

---
