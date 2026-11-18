//! # Exercise 10 — CLI arguments
//!
//! **Goal** — read the program arguments and turn
//! `npm run ex -- ex10 -- add 3 4` into `7`.
//!
//! Support `add`, `sub`, `mul`, `div`, and print a usage message when the
//! arguments are missing or wrong.
//!
//! **Run**  `npm run ex -- ex10 -- add 3 4`
//! **Test** `npm run test:ex -- ex10`
//!
//! **Expected output**
//!
//! ```text
//! $ npm run ex -- ex10 -- add 3 4
//! 7
//!
//! $ npm run ex -- ex10 -- mul 6 7
//! 42
//!
//! $ npm run ex -- ex10
//! error: usage: ex10 <add|sub|mul|div> <a> <b>
//! ```
//!
//! **Hints**
//! - the first `--` belongs to npm, the second one belongs to cargo
//! - your program receives only `["add", "3", "4"]` (after skipping `argv[0]`)
//! - `args[2]` panics when too few arguments are given — use `args.get(2)`
//! - `parse::<f64>()` returns a `Result`; turn the error into a `String`
//! - `"div" if b != 0.0` is a match guard, perfect for the divide-by-zero case
//!
//! Only `run` needs writing; `main` collects the arguments and prints the
//! result or the error.

fn run(args: &[String]) -> Result<f64, String> {
    let _ = args;
    Err("TODO: implement run".to_string())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    match run(&args) {
        Ok(value) => println!("{value}"),
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(items: &[&str]) -> Vec<String> {
        items.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    #[ignore = "exercise not solved yet — run: npm run test:ex -- ex10"]
    fn parses_and_computes() {
        assert_eq!(run(&args(&["add", "3", "4"])), Ok(7.0));
        assert_eq!(run(&args(&["sub", "10", "4"])), Ok(6.0));
        assert_eq!(run(&args(&["mul", "6", "7"])), Ok(42.0));
        assert_eq!(run(&args(&["div", "9", "3"])), Ok(3.0));

        assert!(run(&args(&["div", "1", "0"])).is_err());
        assert!(run(&args(&["pow", "2", "3"])).is_err());
        assert!(run(&args(&["add", "x", "4"])).is_err());
        assert!(run(&args(&["add"])).is_err());
    }
}
