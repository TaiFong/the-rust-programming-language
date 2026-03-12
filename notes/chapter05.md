# Chapter 5 – Using Structs to Structure Data

## Overview

Structs allow you to create **custom data types** by grouping related data together.

They are similar to **structs in C** or **objects without methods**.

Structs help organize data and make programs easier to reason about.

Rust provides three kinds of structs:

* Named-field structs
* Tuple structs
* Unit-like structs

---

## Defining a Struct

A struct is defined using the `struct` keyword.

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```

Each field has a **name and a type**.

---

## Creating an Instance

You create a struct instance by specifying values for each field.

```rust
let user1 = User {
    username: String::from("tai"),
    email: String::from("tai@example.com"),
    sign_in_count: 1,
    active: true,
};
```

Fields are initialized using `field: value`.

---

## Accessing Struct Fields

Fields are accessed with **dot notation**.

```rust
println!("{}", user1.email);
```

---

## Mutable Structs

The entire struct must be mutable to modify its fields.

```rust
let mut user1 = User {
    username: String::from("tai"),
    email: String::from("tai@example.com"),
    sign_in_count: 1,
    active: true,
};

user1.email = String::from("new@email.com");
```

You cannot make only one field mutable.

---

## Struct Update Syntax

You can create a new struct using values from another struct.

```rust
let user2 = User {
    email: String::from("new@example.com"),
    ..user1
};
```

`..user1` copies remaining fields.

Note: ownership of moved fields transfers.

---

## Structs and Functions

Functions can take structs as parameters.

```rust
fn print_email(user: &User) {
    println!("{}", user.email);
}
```

Borrowing (`&User`) avoids moving ownership.

---

## Tuple Structs

Tuple structs group data but **do not name fields**.

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
```

Usage:

```rust
let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

Even if types match, they are **different types**.

---

## Unit-Like Structs

Structs with **no fields**.

```rust
struct AlwaysEqual;
```

Useful when implementing traits later.

---

## Example: Struct With Function

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
```

Usage:

```rust
let rect = Rectangle {
    width: 30,
    height: 50,
};

println!("Area: {}", area(&rect));
```

---

## Debugging Structs

You can print structs using the `Debug` trait.

Add this attribute:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
```

Then print:

```rust
println!("{:?}", rect);
```

Pretty print:

```rust
println!("{:#?}", rect);
```

---

# Key Takeaways

* Structs group related data into custom types.
* Fields are accessed using dot notation.
* Entire structs must be mutable to modify fields.
* Struct update syntax allows reuse of existing values.
* Tuple structs provide named types without field names.
* Unit-like structs define types with no stored data.
* `#[derive(Debug)]` allows easy printing for debugging.

---
