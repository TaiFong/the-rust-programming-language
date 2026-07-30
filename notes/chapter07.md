# Chapter 7 – Packages, Crates, and Modules

## Overview

As Rust projects grow, organizing code becomes essential.

Rust's module system helps structure code into **packages**, **crates**, and **modules**, while controlling visibility through **privacy** and **paths**.

The `use` keyword simplifies long paths, and modules can be split across multiple files to keep projects maintainable.

---

## Packages and Crates

A **package** is a Cargo project managed by `Cargo.toml`.

A package can contain:

- One library crate (`src/lib.rs`)
- Zero or more binary crates (`src/main.rs` or `src/bin/`)

A **crate** is the smallest compilation unit in Rust.

---

## Modules

Modules organize related code into namespaces.

```rust
mod front_of_house {
    fn add_to_waitlist() {}
}
```

Modules help separate functionality and prevent naming conflicts.

---

## Privacy

By default, everything in Rust is **private**.

Use the `pub` keyword to make items accessible outside their module.

```rust
pub fn greet() {
    println!("Hello!");
}
```

Only public items can be accessed from other modules.

---

## Paths

A path specifies where an item exists in the module tree.

Absolute path:

```rust
crate::front_of_house::hosting::add_to_waitlist();
```

Relative path:

```rust
front_of_house::hosting::add_to_waitlist();
```

`crate::` starts from the crate root.

---

## Parent Modules with `super`

`super` refers to the parent module.

```rust
super::deliver_order();
```

Useful when accessing items defined one level above.

---

## Bringing Paths into Scope with `use`

The `use` keyword imports items into the current scope.

```rust
use crate::front_of_house::hosting;

hosting::add_to_waitlist();
```

This reduces repetition and improves readability.

---

## Nested Imports

Multiple imports can be grouped together.

```rust
use std::{cmp::Ordering, io};
```

`self` can also be imported alongside module members.

```rust
use std::io::{self, Write};
```

---

## Glob Operator

The `*` operator imports everything from a module.

```rust
use std::collections::*;
```

Useful in some cases, but explicit imports are generally preferred.

---

## Separating Modules into Files

Large modules can be moved into separate files.

Example:

```text
src/
├── main.rs
├── front_of_house.rs
└── back_of_house.rs
```

Splitting modules keeps projects organized and easier to maintain.

---

# Key Takeaways

- A package is a Cargo project.
- A crate is the smallest compilation unit.
- Modules organize related code into namespaces.
- Items are private by default.
- `pub` exposes items outside a module.
- `crate::` begins at the crate root.
- `super::` refers to the parent module.
- `use` shortens long paths.
- Modules can be split into separate files as projects grow.

---
