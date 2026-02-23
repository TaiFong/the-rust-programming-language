# Chapter 3 – Common Programming Concepts

## Overview

Chapter 3 introduces Rust’s core building blocks:

* Variables & mutability
* Constants
* Data types (scalar + compound)
* Functions
* Expressions vs statements
* Control flow (`if`, `loop`, `while`, `for`)

This chapter establishes how Rust structures programs safely and predictably.

---

## Variables & Mutability

```rust
let x = 5;        // immutable
let mut y = 10;   // mutable
```

* Variables are immutable by default.
* `mut` allows reassignment.
* Encourages safer code by default.

---

## Constants

```rust
const MAX_POINTS: u32 = 100_000;
```

* Must declare type
* Compile-time constant
* Cannot use `mut`
* SCREAMING_SNAKE_CASE convention

---

## Shadowing

```rust
let x = 5;
let x = x + 1;
```

* Creates a new variable with same name
* Can change type
* Different from `mut`

Example:

```rust
let spaces = "   ";
let spaces = spaces.len();
```

---

# Data Types

Rust is statically typed.

## Scalar Types

**Integers**

* `i32` (default)
* `u32`
* `isize`, `usize`

```rust
let x: i32 = 42;
```

**Floating-Point**

* `f64` (default)
* `f32`

```rust
let pi = 3.14;
```

**Boolean**

```rust
let t = true;
```

**Character (Unicode)**

```rust
let c = 'z';
```

---

## Compound Types

### Tuples

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup;
```

* Fixed size
* Mixed types

---

### Arrays

```rust
let a = [1, 2, 3, 4, 5];
```

* Fixed size
* Same type
* Stack allocated
* Out-of-bounds → panic

---

# Functions

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y
}
```

Rules:

* `fn` keyword
* Parameters require types
* Last expression (no `;`) is return value

---

## Statements vs Expressions

Rust is expression-based.

```rust
let y = {
    let x = 3;
    x + 1
};
```

* Expressions return values
* Adding `;` turns it into a statement (`()`)

---

# Control Flow

## `if`

```rust
if number < 5 {
    println!("small");
}
```

* Condition must be `bool`
* No implicit truthiness

`if` is an expression:

```rust
let number = if condition { 5 } else { 6 };
```

Both branches must return same type.

---

## Loops

### `loop`

```rust
loop {
    break;
}
```

Can return value:

```rust
let result = loop {
    break 5;
};
```

---

### `while`

```rust
while number > 0 {
    number -= 1;
}
```

---

### `for`

```rust
for n in 1..=5 {
    println!("{}", n);
}
```

* `1..5` → 1–4
* `1..=5` → 1–5

---

# Key Takeaways

* Rust is expression-oriented.
* Variables are immutable by default.
* Shadowing is powerful and type-safe.
* Functions return the last expression.
* Control flow constructs return values.
* Arrays are fixed-size and memory-safe.
* Rust enforces correctness at compile time.

---
