# Chapter 6 – Enums and Pattern Matching

## Overview

Enums allow you to define a type that can be **one of several possible variants**.

They are useful when a value can have **multiple forms**, unlike structs which have fixed fields.

Pattern matching (`match`) lets you **handle each variant explicitly**.

Rust also provides shortcuts like `if let` and `let...else` for more concise control flow.

---

## Defining an Enum

An enum is defined using the `enum` keyword.

```rust
enum IpAddrKind {
    V4,
    V6,
}
```

Each variant represents a **possible value** of the enum.

---

## Enum with Data

Enums can store data directly in their variants.

```rust
enum IpAddr {
    V4(String),
    V6(String),
}
```

Each variant can hold **different types and amounts of data**.

More complex example:

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

Variants can be:

* No data (`Quit`)
* Struct-like (`Move`)
* Tuple-like (`Write`, `ChangeColor`)

---

## Using Enums

Creating enum values:

```rust
let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));
```

Enums are often used with `match` to handle each case.

---

## The `match` Control Flow Construct

`match` allows you to compare a value against patterns.

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

Each branch is called an **arm**.

---

## Match Must Be Exhaustive

Rust requires handling **all possible cases**.

```rust
match coin {
    Coin::Penny => 1,
    _ => 0,
}
```

`_` is a **catch-all pattern**.

---

## Binding Values in Patterns

You can extract data from enum variants.

```rust
enum Coin {
    Quarter(String),
}

fn describe(coin: Coin) {
    match coin {
        Coin::Quarter(state) => println!("State quarter from {}", state),
    }
}
```

`state` captures the inner value.

---

## Matching with Option<T>

`Option<T>` is a common enum:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

Example:

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}
```

Forces you to handle missing values safely.

---

## Concise Control Flow with `if let`

`if let` is a shorter way to match a single pattern.

```rust
let some_value = Some(3);

if let Some(x) = some_value {
    println!("{}", x);
}
```

Equivalent to a `match` with one arm and `_`.

---

## Using `else` with `if let`

```rust
if let Some(x) = some_value {
    println!("{}", x);
} else {
    println!("No value");
}
```

---

## `let...else`

`let...else` lets you **early-exit** if a pattern doesn't match.

```rust
let Some(x) = some_value else {
    return;
};

println!("{}", x);
```

Useful for cleaner error handling.

---

# Key Takeaways

* Enums define a type with multiple possible variants.
* Variants can store different types and amounts of data.
* `match` enables powerful pattern matching on enums.
* Matches must be exhaustive (handle all cases).
* Pattern matching can extract values from variants.
* `Option<T>` is a key enum for handling absence safely.
* `if let` simplifies matching a single case.
* `let...else` provides clean early-exit control flow.

---

