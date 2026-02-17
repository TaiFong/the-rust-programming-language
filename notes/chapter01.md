### Chapter 1 – Getting Started

**Summary:**  
Introduction to Rust, installing tools, and running basic programs.

**Key Concepts:**

- Rust provides modern developer tools for systems programming:
    
    - **Cargo** – dependency management and build tool
        
    - **rustfmt** – code formatting tool
        
    - **Rust Language Server** – IDE integration
        
    - **rustc** – Rust compiler
        
- Syntax comparison to C
    

**Example:**

**C:**

```c
int main() {
    printf("Hello, world!\n");
}
```

**Rust:**

```rust
fn main() {
    println!("Hello, world!");
}
```

**Compiling & Running Programs:**

```shell
# C
$ gcc -o main main.c
$ ./main

# Rust
$ rustc main.rs
$ ./main
```

**Notes & Observations:**

- `rustup doc --book` opens the book locally in browser (`.html`)
    
- Rust is a statically typed compiled language similar to C/C++
    
- **Cargo.toml File:**  
    Open `Cargo.toml` in your preferred text editor. It should look similar to the following:
    
    ```toml
    [package]
    name = "hello_cargo"
    version = "0.1.0"
    edition = "2024"
    
    [dependencies]
    ```
    
    - **Purpose:** Generated automatically by `cargo new`, it defines your Rust project’s metadata and dependencies.
        
    - **Format:** Written in [_TOML_](https://toml.io/) (_Tom’s Obvious, Minimal Language_), Cargo’s configuration format
        
    - **Reference:** [Listing 1-2 – Contents of Cargo.toml](https://doc.rust-lang.org/stable/book/ch01-03-hello-cargo.html#listing-1-2)
        

---
