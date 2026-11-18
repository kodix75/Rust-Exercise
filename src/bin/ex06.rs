//! # Exercise 06 — Guess the number ⭐
//!
//! **Goal** — the first interactive program. The secret is hardcoded; the
//! program loops, reads a guess from the keyboard, and answers `too high`,
//! `too low` or `win`, counting the attempts.
//!
//! **Run**  `npm run ex -- ex06`
//! **Test** `npm run test:ex -- ex06`
//!
//! **Example session**
//!
//! ```text
//! Guess the number (1-100)
//! 50
//! too high
//! 25
//! too low
//! 42
//! win in 3 attempts
//! ```
//!
//! **Hints**
//! - `read_line` keeps the newline: `input.trim()` removes it
//! - `parse()` returns a `Result`; match on `Ok` / `Err` instead of `unwrap`
//! - `continue` skips to the next loop iteration on bad input
//! - `break` leaves the loop when the guess is right
//!
//! Only `check` needs writing — `main` already contains the input loop.

use std::io;

/// Compare a guess with the secret: "high", "low" or "win".
fn check(guess: u32, secret: u32) -> &'static str {
    let _ = (guess, secret);
    "unimplemented" // TODO: return "high", "low" or "win"
}

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

        match check(guess, secret) {
            "high" => println!("too high"),
            "low" => println!("too low"),
            "win" => {
                println!("win in {attempts} attempts");
                break;
            }
            other => println!("{other}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "exercise not solved yet — run: npm run test:ex -- ex06"]
    fn compares_guess_with_secret() {
        assert_eq!(check(50, 42), "high");
        assert_eq!(check(25, 42), "low");
        assert_eq!(check(42, 42), "win");
    }
}
