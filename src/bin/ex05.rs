//! # Exercise 05 — Vec statistics
//!
//! **Goal** — for `let v = vec![5, 2, 9, 1, 7, 2];` compute and print:
//! the **mean**, the **minimum**, the **maximum**, a **sorted copy**, and how
//! many times the value `2` appears.
//!
//! **Run**  `npm run ex -- ex05`
//! **Test** `npm run test:ex -- ex05`
//!
//! **Expected output**
//!
//! ```text
//! values = [5, 2, 9, 1, 7, 2]
//! mean   = 4.33
//! min    = 1
//! max    = 9
//! sorted = [1, 2, 2, 5, 7, 9]
//! count  = 2
//! ```
//!
//! **Hints**
//! - `v.iter().sum::<i32>()` gives the sum; `v.len()` the count
//! - `v.iter().min()` returns `Option<&i32>` because a vector can be empty
//! - for a sorted **copy**: `let mut s = v.to_vec(); s.sort();`
//! - `v.iter().filter(|x| **x == 2).count()`
//!
//! The functions take `&[i32]` (a slice) so they work with any vector.

fn mean(v: &[i32]) -> f64 {
    let _ = v;
    0.0 // TODO: sum as f64 / len as f64
}

fn min(v: &[i32]) -> Option<i32> {
    let _ = v;
    None // TODO: v.iter().min().copied()
}

fn max(v: &[i32]) -> Option<i32> {
    let _ = v;
    None // TODO: v.iter().max().copied()
}

fn sorted(v: &[i32]) -> Vec<i32> {
    let _ = v;
    Vec::new() // TODO: copy, then sort
}

fn count_of(v: &[i32], value: i32) -> usize {
    let _ = (v, value);
    0 // TODO: filter and count
}

fn main() {
    let v = vec![5, 2, 9, 1, 7, 2];

    println!("values = {v:?}");
    println!("mean   = {:.2}", mean(&v));
    println!("min    = {}", min(&v).unwrap_or_default());
    println!("max    = {}", max(&v).unwrap_or_default());
    println!("sorted = {:?}", sorted(&v));
    println!("count  = {}", count_of(&v, 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "exercise not solved yet — run: npm run test:ex -- ex05"]
    fn statistics_match() {
        let v = [5, 2, 9, 1, 7, 2];
        assert!((mean(&v) - 4.333_333).abs() < 0.001);
        assert_eq!(min(&v), Some(1));
        assert_eq!(max(&v), Some(9));
        assert_eq!(sorted(&v), vec![1, 2, 2, 5, 7, 9]);
        assert_eq!(count_of(&v, 2), 2);
    }
}
