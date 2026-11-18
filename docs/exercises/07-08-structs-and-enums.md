# Exercises 07–08 — Structs and enums

> The two ways to model data in Rust: a struct groups values, an enum lists possibilities.
>
> Run one: `npm run ex -- ex07` · Test one: `npm run test:ex -- ex07`

**In this file:** Ex 07 · Ex 08

---

## Ex 07 — BankAccount struct

> **Concepts:** `struct` · `impl` · `&self` vs `&mut self` · `Result`
> **File:** `src/bin/ex07.rs` · **Run:** `npm run ex -- ex07`

### Goal

Model a bank account with data **and** behaviour.

```rust
struct BankAccount {
    owner: String,
    balance: f64,
}
```

Methods:

| Method | Signature | Behaviour |
|---|---|---|
| `new` | `fn new(owner: &str, balance: f64) -> Self` | create an account |
| `deposit` | `fn deposit(&mut self, amount: f64)` | add money |
| `withdraw` | `fn withdraw(&mut self, amount: f64) -> Result<(), String>` | error if not enough money |
| `display` | `fn display(&self)` | print owner and balance |

### Starter

```rust
struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn new(owner: &str, balance: f64) -> Self {
        Self {
            owner: owner.to_string(),
            balance,
        }
    }

    fn deposit(&mut self, amount: f64) {
        // ...
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> {
        // Err(...) if amount > self.balance
        Ok(())
    }

    fn display(&self) {
        println!("{}: {:.2} CHF", self.owner, self.balance);
    }
}

fn main() {
    let mut account = BankAccount::new("Aidan", 100.0);

    account.display();
    account.deposit(50.0);
    account.display();

    match account.withdraw(500.0) {
        Ok(()) => println!("withdrawal ok"),
        Err(e) => println!("error: {e}"),
    }

    account.display();
}
```

### Expected output

```text
Aidan: 100.00 CHF
Aidan: 150.00 CHF
error: insufficient funds (balance 150.00, requested 500.00)
Aidan: 150.00 CHF
```

### Hints

<details>
<summary>Hint — <code>&self</code> or <code>&mut self</code>?</summary>

- `display` only reads → `&self`
- `deposit` and `withdraw` change `balance` → `&mut self`

The compiler will tell you immediately if you pick the wrong one.
</details>

<details>
<summary>Hint — the error message</summary>

```rust
if amount > self.balance {
    return Err(format!(
        "insufficient funds (balance {:.2}, requested {:.2})",
        self.balance, amount
    ));
}
self.balance -= amount;
Ok(())
```

`format!` builds a `String` (like `println!` but returns it).
</details>

<details>
<summary>Hint — <code>Self</code> vs <code>BankAccount</code></summary>

Inside an `impl` block, `Self` means "the type being implemented". Both work;
`Self` keeps the code correct if you rename the struct later.
</details>

### Read & search

- [Structs](https://doc.rust-lang.org/book/ch05-00-structs.html)
- [Methods](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
- [Field init shorthand](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#using-the-field-init-shorthand)

### Stretch

- Add `fn transfer(to: &mut BankAccount, amount: f64) -> Result<(), String>`.
  Careful: you cannot borrow `self` mutably twice.
- Reject negative deposits and withdrawals (`Err` for `amount <= 0.0`).
- Replace `f64` with an integer number of cents (`u64`) — how real banks do it.

---

## Ex 08 — Enum calculator

> **Concepts:** `enum` · `match` · `Result` · exhaustive patterns
> **File:** `src/bin/ex08.rs` · **Run:** `npm run ex -- ex08`

### Goal

Represent the four operations as an enum, and apply them with `match`.

```rust
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

fn apply(op: Op, a: f64, b: f64) -> Result<f64, String>
```

Division by zero returns `Err("cannot divide by zero")`.

Call all four operations and print the results.

### Starter

```rust
#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

fn apply(op: Op, a: f64, b: f64) -> Result<f64, String> {
    match op {
        Op::Add => Ok(a + b),
        Op::Sub => Ok(a - b),
        Op::Mul => Ok(a * b),
        Op::Div => {
            if b == 0.0 {
                Err("cannot divide by zero".to_string())
            } else {
                Ok(a / b)
            }
        }
    }
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
```

### Expected output

```text
Add: 16
Sub: 8
Mul: 48
Div: 3
```

### Hints

<details>
<summary>Hint — why an enum instead of a string?</summary>

`"add"`, `"Add"`, `"plus"`, `"+"` are all easy to mistype and impossible for
the compiler to check. An `enum` has exactly four values, and `match` forces
you to handle every one — add a variant later and the compiler lists every
place you forgot.
</details>

<details>
<summary>Hint — what does <code>#[derive(Debug, Clone, Copy)]</code> do?</summary>

- `Debug` → lets you print with `{op:?}`
- `Clone`, `Copy` → the enum can be copied instead of moved, so you can use
  `op` again after passing it to `apply`
</details>

<details>
<summary>Hint — division by zero</summary>

`4.0 / 0.0` in Rust does **not** panic for floats: it gives `inf`. That is
why the exercise asks you to check `b == 0.0` yourself. (Integer division by
zero *does* panic.)
</details>

### Read & search

- [Enums](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)
- [`match`](https://doc.rust-lang.org/book/ch06-02-match.html)
- [Derivable traits](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html)

### Stretch

- Parse an operator from a string: `fn parse_op(s: &str) -> Result<Op, String>`.
- Add `Pow` and `Mod` variants — the compiler shows you the `match` to update.

---
