//! # Exercise 04 — Factorial
//!
//! **Goal** — write factorial **twice**:
//! - `factorial_iter(n: u64) -> u64` with a loop
//! - `factorial_rec(n: u64) -> u64` with recursion
//!
//! Print both for `n = 5` and `n = 10`. They must agree.
//!
//! `5! = 5 × 4 × 3 × 2 × 1 = 120`
//!
//! **Run**  `npm run ex -- ex04`
//! **Test** `npm run test:ex -- ex04`
//!
//! **Expected output**
//!
//! ```text
//! 5! = 120 (iter) / 120 (rec)
//! 10! = 3628800 (iter) / 3628800 (rec)
//! ```
//!
//! **Hints**
//! - the loop version: start at `1` and multiply by every number up to `n`
//! - the recursive version needs a **base case**: `if n <= 1 { 1 }`
//! - the last expression has no semicolon — it is the return value
//! - `u64` overflows at `21!`; try `20!` then `21!` to see the panic

fn factorial_iter(n: u64) -> u64 {
    let _ = n;
    0 // TODO: loop from 1..=n and multiply
}

fn factorial_rec(n: u64) -> u64 {
    let _ = n;
    0 // TODO: base case, then n * factorial_rec(n - 1)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "exercise not solved yet — run: npm run test:ex -- ex04"]
    fn both_versions_agree() {
        assert_eq!(factorial_iter(5), 120);
        assert_eq!(factorial_rec(5), 120);
        assert_eq!(factorial_iter(10), 3_628_800);
        assert_eq!(factorial_rec(10), 3_628_800);
        assert_eq!(factorial_iter(0), 1);
    }
}
