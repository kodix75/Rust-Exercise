# Rust — basic syntax

A one-page reference for the exercises. Everything here is covered by the
[Rust Book](https://doc.rust-lang.org/book/); open it when you want the full
explanation.

- [Variables](#variables)
- [Types](#types)
- [Printing](#printing)
- [Conditions](#conditions)
- [Loops](#loops)
- [Collections](#collections)
- [Functions](#functions)
- [Structs](#structs)
- [Enums & match](#enums--match)
- [Option & Result](#option--result)
- [Ownership & borrowing](#ownership--borrowing)

---

## Variables

```rust
let x = 5;              // immutable
let mut y = 10;         // mutable
y += 1;
let x = "shadowed";     // a new binding with the same name is allowed
```

## Types

```rust
let a: i32 = -42;       // i8 i16 i32 i64 i128 isize (signed)
let b: u64 = 42;        // u8 u16 u32 u64 u128 usize (unsigned)
let c: f64 = 3.14;      // floats: f32, f64
let d: bool = true;
let e: char = 'é';      // single character, single quotes
let s: &str = "borrowed";
let owned: String = String::from("owned");
```

## Printing

```rust
println!("x = {x}, y = {y}");   // inline variables
println!("{c:.2}");             // 2 decimals
eprintln!("goes to stderr");
```

## Conditions

```rust
if x > 3 {
    println!("big");
} else {
    println!("small");
}

// `if` is an expression
let label = if x > 3 { "big" } else { "small" };
```

## Loops

```rust
for i in 0..5  { println!("{i}"); }   // 0 1 2 3 4
for i in 0..=5 { println!("{i}"); }   // 0 1 2 3 4 5

let mut n = 0;
while n < 3 {
    n += 1;
}

let answer = loop {          // `loop` can return a value
    break 42;
};
```

## Collections

```rust
let arr = [1, 2, 3];             // fixed size
let tup = (1, "two", 3.0);       // access with tup.0, tup.1
let mut v: Vec<i32> = vec![1, 2, 3];
v.push(4);

for value in &v {
    println!("{value}");
}
```

## Functions

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b                 // no semicolon: this is the return value
}

fn main() {
    println!("{}", add(2, 3));
}
```

Functions live at the top level — never inside `main`.

## Structs

```rust
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn dist_to_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

let p = Point::new(3.0, 4.0);
println!("{}", p.dist_to_origin());   // 5
```

## Enums & match

```rust
enum Shape {
    Circle(f64),
    Square(f64),
}

fn area(s: &Shape) -> f64 {
    match s {
        Shape::Circle(r) => 3.14159 * r * r,
        Shape::Square(a) => a * a,
    }
}
```

`match` must cover every variant.

## Option & Result

```rust
fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 { None } else { Some(a / b) }
}

match divide(1.0, 0.0) {
    Some(v) => println!("{v}"),
    None => println!("cannot divide by zero"),
}

fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse::<i32>()
}
```

`?` propagates an error to the caller:

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let text = std::fs::read_to_string("notes.txt")?;
    println!("{text}");
    Ok(())
}
```

## Ownership & borrowing

```rust
let s1 = String::from("hi");
let s2 = s1;              // s1 is MOVED, do not use it afterwards
let s3 = s2.clone();      // deep copy

fn len(s: &String) -> usize {   // borrow instead of moving
    s.len()
}

println!("{}", len(&s2));
```

- `&x` — read-only borrow
- `&mut x` — mutable borrow (only one at a time)
- `.clone()` — copy heap data when you really need two owners

---

## Compiler messages you will meet

| Message | Fix |
|---|---|
| `cannot assign to x, as it is not declared as mutable` | `let mut x = …` |
| `use of moved value` | `.clone()` or borrow with `&` |
| `mismatched types` | cast (`x as f64`) or parse with the right type |
| `cannot find function x` | define it at top level, outside `main` |
| `warning: unused variable` | use it, prefix `_x`, or delete it |

Fix the **first** error first — the rest are often consequences of it.
