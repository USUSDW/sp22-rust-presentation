# Rust

Rust is a language designed with memory safety in mind, along with a bunch of cool features like pattern matching, a trait system, and type-encoded error handling. This repo will focus on the ownership system of Rust.

Links:
* [Rust Homepage](https://www.rust-lang.org/)
* [Rust Get Started](https://www.rust-lang.org/learn/get-started)
* [The Rust Programming Language (aka The Book)](https://doc.rust-lang.org/book/)

## Repo info

This repo has a C++ example to demonstrate using invalid memory. The rest is Rust code mostly taken from [Chapter 4](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html) of the book.

### C++ example

The C++ example uses CMake as a build system. I will assume you either know or can learn how to use CMake, as this repo is about Rust ownership.

### Rust examples

You can run the Rust examples by running `cargo run --example [example_name]`, e.g.:

> `cargo run --example 04_ownership_functions_moves_and_copies`

The examples are in the `rust-example/examples` folder.
