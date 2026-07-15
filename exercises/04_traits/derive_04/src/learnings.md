# Understanding `#[derive]` in Rust

## What is `#[derive]`?

In Rust, **`#[derive]`** is a **built-in attribute** (specifically a *procedural macro*) that tells the compiler to automatically generate the implementation of certain traits for your custom data types (structs, enums, or unions).

> **Important distinction:** `derive` is **not** a trait itself. It is a **tool** that implements existing traits for you.

Instead of writing repetitive boilerplate code (e.g., manually implementing `Debug`, `Clone`, or `PartialEq`), you simply add `#[derive(TraitName)]` above your type definition, and the compiler handles the rest.

---

## Basic Syntax

```rust
#[derive(Debug, Clone, PartialEq)]
struct User {
    id: u32,
    name: String,
    active: bool,
}