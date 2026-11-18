//! # Exercise 09 — Read a file
//!
//! **Goal** — read a text file and print the number of **lines**, **words**
//! and **characters**. Use `std::fs::read_to_string` and the `?` operator.
//!
//! **Run**  `npm run ex -- ex09` (reads `sample.txt` if no path is given)
//! **Test** `npm run test:ex -- ex09`
//!
//! **Expected output** (for `sample.txt` containing two lines)
//!
//! ```text
//! file   = sample.txt
//! lines  = 2
//! words  = 6
//! chars  = 28
//! ```
//!
//! **Hints**
//! - `?` means "if this is an `Err`, return it now; otherwise give me the value"
//! - for `?` to work, the function must return a `Result`
//! - `text.lines().count()` — lines
//! - `text.split_whitespace().count()` — words
//! - `text.chars().count()` — characters (`len()` counts **bytes**)
//!
//! Only `stats` needs writing; `main` handles the argument and printing.

use std::env;
use std::io;
// TODO: `use std::fs;` when you implement `stats`.

/// Return `(lines, words, chars)` for the file at `path`.
fn stats(path: &str) -> Result<(usize, usize, usize), io::Error> {
    let _ = path;
    Err(io::Error::other("TODO: implement stats"))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::args()
        .nth(1)
        .unwrap_or_else(|| "sample.txt".to_string());

    let (lines, words, chars) = stats(&path)?;

    println!("file   = {path}");
    println!("lines  = {lines}");
    println!("words  = {words}");
    println!("chars  = {chars}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    #[ignore = "exercise not solved yet — run: npm run test:ex -- ex09"]
    fn counts_lines_words_and_chars() {
        let path = env::temp_dir().join(format!("rust-ex09-{}.txt", std::process::id()));
        fs::write(&path, "Rust is fast.\nRust is safe.\n").expect("temp file is writable");

        let result = stats(path.to_str().expect("utf-8 path")).expect("stats succeeds");
        assert_eq!(result, (2, 6, 28));

        let _ = fs::remove_file(&path);
    }
}
