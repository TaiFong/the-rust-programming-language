# Chapter 4 – Understanding Ownership (One Pager)

## Overview

Ownership is Rust’s core memory model.

Instead of garbage collection or manual `free`, Rust enforces memory safety at **compile time** using:

* Ownership
* Borrowing
* References
* Slices

This prevents:

* Double free
* Use-after-free
* Data races
* Dangling pointers

---

## Ownership Rules

1. Each value has a single owner.
2. Only one owner at a time.
3. When the owner goes out of scope, the value is dropped.

```rust
{
    let s = String::from("hello");
} // s is dropped here
```

---

## Stack vs Heap

* **Stack** → fixed size, fast (`i32`, `bool`)
* **Heap** → dynamic size (`String`, `Vec`)

```rust
let x = 5;                      // stack
let s = String::from("hello");  // heap
```

Heap data follows ownership rules strictly.

---

## Move Semantics

Primitive types copy:

```rust
let x = 5;
let y = x; // copy
```

Heap types move:

```rust
let s1 = String::from("hello");
let s2 = s1; // move
```

`s1` becomes invalid.
Prevents double free.

---

## Clone

Explicit deep copy:

```rust
let s1 = String::from("hello");
let s2 = s1.clone();
```

Use only when necessary.

---

## Ownership & Functions

Passing a value moves it:

```rust
fn takes(s: String) {
    println!("{}", s);
}
```

Borrow instead:

```rust
fn borrow(s: &String) {
    println!("{}", s);
}
```

References do not transfer ownership.

---

## References & Borrowing

Immutable reference:

```rust
let s = String::from("hello");
let len = calculate(&s);
```

Mutable reference:

```rust
let mut s = String::from("hello");
change(&mut s);
```

Rules:

* Many immutable references allowed.
* Only one mutable reference at a time.
* No mutable + immutable at the same time.

Prevents data races at compile time.

---

## Dangling References (Prevented)

Rust will not allow returning a reference to local data:

```rust
fn dangle() -> &String {
    let s = String::from("hello");
    &s // error
}
```

Memory safety guaranteed.

---

## Slices

Slices reference part of a collection.

```rust
let s = String::from("hello world");
let hello = &s[0..5];
```

String slice type:

```rust
&str
```

Slices do not take ownership.

---

# Key Takeaways

* Ownership replaces garbage collection.
* Values move by default.
* Borrowing enables safe access without transfer.
* Only one mutable reference at a time.
* Rust enforces memory safety at compile time.
* This is the foundation of everything in Rust.
