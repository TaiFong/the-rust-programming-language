# Chapter 2 – Programming a Guessing Game

## Summary:

This chapter introduces core Rust concepts by building a number guessing game.  
You learn how to:

- Accept user input
    
- Generate random numbers
    
- Use variables and mutability
    
- Handle errors
    
- Use pattern matching
    
- Compare values
    
- Work with external crates (`rand`)
    
- Understand basic module paths (`std::io`)
    
- Use `use` and `::`
    

It gradually introduces Rust’s syntax and safety model through practical code.

---

## Key Concepts:

### 1. `use` and the Standard Library (`std`)

- `std` = Rust standard library crate
    
- `use std::io;` brings the `io` module into scope
    
- `::` is the path separator (crate → module → item)
    

Example:

```rust
use std::io;

io::stdin();
```

Without `use`:

```rust
std::io::stdin();
```

---

### 2. Variables and Mutability

By default, variables are immutable.

```rust
let guess = String::new(); // immutable
```

To allow modification:

```rust
let mut guess = String::new(); // mutable
```

`mut` = allow changes to the variable.

---

### 3. Getting User Input

```rust
io::stdin()
    .read_line(&mut guess) // like scanf in c
    .expect("Failed to read line");
```

Breakdown:

- `stdin()` → access terminal input
    
- `.read_line()` → method that writes into a string
    
- `&mut guess` → mutable reference
    
- `.expect()` → crash program if error occurs
    

---

### 4. Error Handling (`Result`)

Many Rust operations return:

```
Result<T, E>
```

Meaning:

- `Ok(T)` → success
    
- `Err(E)` → error
    

`.expect()` unwraps the result or crashes with a message.

---

### 5. Adding an External Crate (`rand`)

Rust uses **Cargo** to manage dependencies.

In `Cargo.toml`:

```toml
rand = "0.8"
```

Import in code:

```rust
use rand::Rng;
```

Generate random number:

```rust
let secret_number = rand::thread_rng().gen_range(1..=100);
```

---

### 6. Shadowing

You can reuse variable names:

```rust
let guess = guess.trim();
let guess: u32 = guess.parse().expect("Please type a number!");
```

This is called **shadowing**.

It creates a new variable with the same name.

---

### 7. Parsing Strings into Numbers

User input is a `String`.

Convert it:

```rust
guess.parse()
```

You must specify type:

```rust
let guess: u32 = guess.parse().expect("Not a number");
```

Rust needs type information to know what to convert into.

---

### 8. Pattern Matching (`match`)

Used to compare values:

```rust
match guess.cmp(&secret_number) {
    Ordering::Less => println!("Too small!"),
    Ordering::Greater => println!("Too big!"),
    Ordering::Equal => println!("You win!"),
}
```

- `cmp()` compares two values
    
- Returns `Ordering` enum
    
- `match` handles each possible case
    

---

### 9. Enums (`Ordering`)

`Ordering` can be:

- `Less`
    
- `Greater`
    
- `Equal`
    

Rust forces you to handle all possibilities.

This makes code safer.

---

### 10. Loops

To keep guessing:

```rust
loop {
    // code here
}
```

Infinite loop until manually broken.

---

## Examples:

### Guessing Game (Core Structure)

```rust
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

---

## Commands / Compilation / Running:

Create project:

```bash
$ cargo new guessing_game
$ cd guessing_game
```

Add dependency:

Edit `Cargo.toml`

```toml
rand = "0.8"
```

Run program:

```bash
$ cargo run
```

Build without running:

```bash
$ cargo build
```

---

## Notes & Observations:

- Rust variables are immutable by default (safer design).
    
- Error handling is explicit (`Result`).
    
- Pattern matching ensures all cases are handled.
    
- Shadowing allows safe type transformation.
    
- `use` shortens module paths.
    
- `::` navigates module hierarchy.
    
- Rust enforces type safety during parsing.
    
- Cargo simplifies dependency management.
    

---
