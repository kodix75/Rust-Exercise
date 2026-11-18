//! # Exercise 01 — Print & math
//!
//! **Goal** — write four functions, each taking two `f64` and returning an
//! `f64`: `add`, `sub`, `mul`, `div`. Then print all four results for
//! `a = 12.0` and `b = 4.0`.
//!
//! **Run**  `npm run ex -- ex01`
//! **Test** `npm run test:ex -- ex01`
//!
//! **Expected output**
//!
//! ```text
//! sum      = 16
//! diff     = 8
//! product  = 48
//! quotient = 3
//! ```
//!
//! **Hints**
//! - the return value is the last expression, **without** a semicolon
//! - `12` is an integer, `12.0` is a float — these functions need `f64`
//! - the extra spaces in the strings are what line the `=` up
//!
//! The test at the bottom is marked `#[ignore]` so `npm test` stays green
//! while the exercise is unsolved; `npm run test:ex -- ex01` runs it anyway.

fn add(a: f64, b: f64) -> f64 {
    let _ = (a, b);
    0.0 // TODO: replace with `a + b`
}

fn sub(a: f64, b: f64) -> f64 {
    let _ = (a, b);
    0.0 // TODO: replace with `a - b`
}

fn mul(a: f64, b: f64) -> f64 {
    let _ = (a, b);
    0.0 // TODO: replace with `a * b`
}

fn div(a: f64, b: f64) -> f64 {
    let _ = (a, b);
    0.0 // TODO: replace with `a / b`
}

fn main() {
    let a = 12.0;
    let b = 4.0;

    println!("sum      = {}", add(a, b));
    println!("diff     = {}", sub(a, b));
    println!("product  = {}", mul(a, b));
    println!("quotient = {}", div(a, b));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "exercise not solved yet — run: npm run test:ex -- ex01"]
    fn four_operations() {
        assert_eq!(add(12.0, 4.0), 16.0);
        assert_eq!(sub(12.0, 4.0), 8.0);
        assert_eq!(mul(12.0, 4.0), 48.0);
        assert_eq!(div(12.0, 4.0), 3.0);
    }
}
