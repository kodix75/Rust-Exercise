//! # Exercise 03 — FizzBuzz
//!
//! **Goal** — print the numbers 1 to 30, but:
//! - multiples of **3** print `Fizz`
//! - multiples of **5** print `Buzz`
//! - multiples of **both** print `FizzBuzz`
//! - everything else prints the number
//!
//! **Run**  `npm run ex -- ex03`
//! **Test** `npm run test:ex -- ex03`
//!
//! **Expected output (first lines)**
//!
//! ```text
//! 1
//! 2
//! Fizz
//! 4
//! Buzz
//! Fizz
//! 7
//! 8
//! Fizz
//! Buzz
//! 11
//! Fizz
//! 13
//! 14
//! FizzBuzz
//! ```
//!
//! **Hints**
//! - test the "both" case **first**, otherwise 15 prints `Fizz`
//! - `n % 15 == 0` means divisible by 3 and 5 at the same time
//! - return a `String`, then `main` prints it — that is what the test checks

fn fizzbuzz(n: u32) -> String {
    let _ = n;
    String::new() // TODO: return "Fizz", "Buzz", "FizzBuzz" or n.to_string()
}

fn main() {
    for n in 1..=30 {
        println!("{}", fizzbuzz(n));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "exercise not solved yet — run: npm run test:ex -- ex03"]
    fn fizz_buzz_and_both() {
        assert_eq!(fizzbuzz(3), "Fizz");
        assert_eq!(fizzbuzz(5), "Buzz");
        assert_eq!(fizzbuzz(15), "FizzBuzz");
        assert_eq!(fizzbuzz(7), "7");
    }
}
