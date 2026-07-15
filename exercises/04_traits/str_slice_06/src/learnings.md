# Why Returning `&String` is Considered Wrong in Rust

## The Short Answer

Technically, **returning `&String` is not *wrong* in the sense that it will compile and run perfectly fine.** 

However, in the Rust community, it is considered **non-idiomatic**, **inflexible**, and **poor API design**. This document explains exactly why `&str` is the correct, idiomatic choice for accessor methods.

---

## The Problem: A Ticket Struct with Accessors

Consider this struct and its accessor methods:

```rust
pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    // ❌ Non-idiomatic: Returning &String
    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn status(&self) -> &String {
        &self.status
    }
}
```


# Summary Table: `&String` vs `&str` in Rust Accessors

This table summarizes why returning `&str` is preferred over returning `&String` in Rust accessor methods.

---

## 📊 Comparison: `&String` vs `&str`

| Aspect | Returning `&String` | Returning `&str` |
| :--- | :--- | :--- |
| **Indirection Level** | Double (pointer to struct, then pointer to heap) | Single (pointer directly to heap data) |
| **Encapsulation** | Exposes internal `String` type (leaky abstraction) | Hides internal implementation details |
| **Flexibility for Callers** | Forces caller to deal with `&String` (must call `.as_str()` or dereference) | Works everywhere `&str` is expected (e.g., `println!`, comparisons, libraries) |
| **Idiomatic Rust** | ❌ Considered a code smell / poor practice | ✅ The standard, idiomatic way for read-only accessors |
| **Deref Coercion** | Prevents automatic coercion to `&str` | Leverages `Deref` coercion (automatically converts `&String` to `&str` in the body) |
| **Changes to Function Body** | N/A | No changes needed! Just change the signature from `-> &String` to `-> &str` |
| **Future-Proofing** | Breaks if you change internal storage (e.g., to `Box<str>`) | Remains stable regardless of internal storage type |

---

## 🎯 The Golden Rule

> **For accessor methods that only read data, always return `&str` over `&String`.**

The caller doesn't care *how* you store the string; they only care about reading its contents. Keep your API flexible, encapsulated, and idiomatic!

---

## ✅ Correct Implementation Example

```rust
impl Ticket {
    // ✅ Idiomatic: Return &str
    pub fn title(&self) -> &str {
        &self.title   // Automatically coerces &String -> &str
    }
}