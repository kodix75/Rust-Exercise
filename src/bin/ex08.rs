//! # Exercise 08 — Enum calculator
//!
//! **Goal** — represent the four operations as an enum, and apply them with
//! `match`. Division by zero returns an error instead of `inf`.
//!
//! ```text
//! enum Op { Add, Sub, Mul, Div }
//! fn apply(op: Op, a: f64, b: f64) -> Result<f64, String>
//! ```
//!
//! **Run**  `npm run ex -- ex08`
//! **Test** `npm run test:ex -- ex08`
//!
//! **Expected output**
//!
//! ```text
//! Add: 16
//! Sub: 8
//! Mul: 48
//! Div: 3
//! ```
//!
//! **Hints**
//! - `match` must cover every variant — that is the point of the enum
//! - `4.0 / 0.0` gives `inf` for floats, it does not panic; check `b == 0.0`
//! - `#[derive(Debug, Clone, Copy)]` lets you print `{op:?}` and reuse `op`

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

fn apply(op: Op, a: f64, b: f64) -> Result<f64, String> {
    let _ = (op, a, b);
    Err("TODO: implement apply".to_string())
}

fn main() {
    let a = 12.0;
    let b = 4.0;

    for op in [Op::Add, Op::Sub, Op::Mul, Op::Div] {
        match apply(op, a, b) {
            Ok(value) => println!("{op:?}: {value}"),
            Err(e) => println!("{op:?}: error — {e}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "exercise not solved yet — run: npm run test:ex -- ex08"]
    fn every_operation() {
        assert_eq!(apply(Op::Add, 12.0, 4.0), Ok(16.0));
        assert_eq!(apply(Op::Sub, 12.0, 4.0), Ok(8.0));
        assert_eq!(apply(Op::Mul, 12.0, 4.0), Ok(48.0));
        assert_eq!(apply(Op::Div, 12.0, 4.0), Ok(3.0));
        assert!(apply(Op::Div, 1.0, 0.0).is_err());
    }
}
