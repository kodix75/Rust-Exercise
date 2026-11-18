//! # Exercise 02 — Mutability & shadowing
//!
//! **Goal** — learn the difference between *changing* a value and creating a
//! *new binding with the same name*.
//!
//! 1. Start with `let n = 10;`
//! 2. Add `n += 1;` — run it and **read the compiler error** before fixing it.
//! 3. Fix it by adding `mut` where `n` is declared.
//! 4. Then shadow it: `let n = n * 2;` and print the result.
//!
//! **Run**  `npm run ex -- ex02`
//! **Test** `npm run test:ex -- ex02`
//!
//! **Expected output**
//!
//! ```text
//! n = 22
//! bump(10) = 11
//! ```
//!
//! **Hints**
//! - `mut` goes on the declaration (`let mut n = 10`), not on the use
//! - shadowing can even change the type: `let n = n.to_string();`
//! - the test below checks `bump`, which must return `n + 1`

fn bump(n: i32) -> i32 {
    let _ = n;
    0 // TODO: replace with `n + 1`
}

fn main() {
    let mut n = 10; // step 3: remove `mut` and read the error from step 2
    n += 1; // step 2: this line does not compile without `mut`

    let n = n * 2; // step 4: shadowing creates a NEW variable

    println!("n = {n}");
    println!("bump(10) = {}", bump(10));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "exercise not solved yet — run: npm run test:ex -- ex02"]
    fn bump_adds_one() {
        assert_eq!(bump(10), 11);
        assert_eq!(bump(0), 1);
        assert_eq!(bump(-1), 0);
    }
}
