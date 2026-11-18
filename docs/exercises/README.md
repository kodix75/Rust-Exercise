# Exercises

Ten small programs, in order. The guides are grouped **two or three per
file** so you can read a whole topic in one go instead of jumping between
ten pages.

| # | Exercise | You practise | Guide |
|:--:|---|---|:--|
| 01 | Print & math | functions, `f64`, `println!` | [01–03 basics](01-03-basics.md) |
| 02 | Mutability & shadowing | `let`, `mut`, shadowing | [01–03 basics](01-03-basics.md) |
| 03 | FizzBuzz | `for`, `if`, `%` | [01–03 basics](01-03-basics.md) |
| 04 | Factorial | loops, recursion | [04–06 loops & input](04-06-loops-and-input.md) |
| 05 | Vec statistics | `Vec`, iterators | [04–06 loops & input](04-06-loops-and-input.md) |
| 06 | Guess the number | `loop`, `Result`, input | [04–06 loops & input](04-06-loops-and-input.md) |
| 07 | BankAccount | `struct`, `impl`, `&mut self` | [07–08 structs & enums](07-08-structs-and-enums.md) |
| 08 | Enum calculator | `enum`, `match`, errors | [07–08 structs & enums](07-08-structs-and-enums.md) |
| 09 | Read a file | `Result`, `?`, file I/O | [09–10 files & args](09-10-files-and-args.md) |
| 10 | CLI arguments | `env::args`, parsing | [09–10 files & args](09-10-files-and-args.md) |

## How an exercise works

Every exercise file **already exists** in `src/bin/` with:

- the full instructions as a comment at the top,
- the function signatures and a `main` that is ready to run,
- a `TODO` where your code goes,
- **one test** that checks your work.

1. Open `src/bin/exNN.rs` and read the comment at the top.
2. Replace the `TODO` with real code.
3. Run it — no cargo knowledge needed:

   ```bash
   npm run ex -- ex01
   ```

4. Check your work:

   ```bash
   npm run test:ex -- ex01
   ```

   The test is green only when the exercise is correct. It is marked
   `#[ignore]` in the file so that `npm test` (the repository check) stays
   green while you are still learning — the exercise runner runs it anyway.

5. Run the whole repository check, then commit:

   ```bash
   npm test
   git add src/bin/ex01.rs
   git commit -m "ex01: print & math"
   ```

## Order matters

Exercises 1–5 are the language basics. 6 introduces input and errors. 7–8 are
the two ways to model data (structs and enums). 9–10 connect the program to
the outside world (files and arguments).

If you only have 20 minutes: do ex01, ex03 and ex05 — they cover the most
ground.

> The first real project lives on the
> [`my_first_project`](https://github.com/kodix75/Rust-Exercise/tree/my_first_project)
> branch.
