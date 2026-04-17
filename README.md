# Shriya-100-exercises-to-learn-Rust

A structured repository of my solutions to the **100 Exercises to Learn Rust** course (`rust-exercises.com/100-exercises`), plus small projects built while learning Rust and Solana development.

## 📌 About this repo

This repository documents my journey from Rust beginner to confident systems‑level programmer, focused on:
- Core Rust concepts (ownership, borrowing, `Result`/`Option`, traits, iterators, concurrency).
- Small CLI tools and on‑the‑way projects.
- Preparation for **Rust + Solana smart‑contract development**.

I’m using this repo as:
- A **learning portfolio** for jobs and internships.
- A **public record** of progress and problem‑solving skills.

## 📚 Source of exercises

- Course: [100 Exercises to Learn Rust – rust-exercises.com](https://rust-exercises.com/100-exercises)  and official Rust docs (https://doc.rust-lang.org/book).
- These exercises are small, incremental problems designed to teach Rust’s syntax, type system, and standard library by doing.

## 📁 Folder structure

- `exercises/`  
  - Contains one file per exercise (e.g., `e001.rs`, `e002.rs`, …, `e100.rs`).  
- `mini_projects/`  
  - Small CLI apps built while learning Rust (e.g., `word_counter`, `todo_cli`).
- `Projects/`
   - Intermediate projects 
- `docs/`  
  - Learning notes and plan (`learning_plan.md`).

## 🧩 Example project: Word Counter

- Path: `mini_projects/word_counter/`  
- What it does: reads a file or stdin and counts words using Rust’s `std::io` and iterators.  
- Goal: practice ownership, `Result` handling, and CLI‑style logic.

## 🧩 Example project: Todo CLI

- Path: `mini_projects/todo_cli/`  
- What it does: a simple in‑memory todo manager (add, list, mark done).  
- Goal: practice structs, enums, pattern matching, and basic application design.

## 🧪 How to compile and run

From the root of the repo:

```bash
# Example: run a single exercise
rustc exercises/e001.rs -o e001
./e001

# Example: build a mini_project (Cargo project recommended later)
cd mini_projects/word_counter
cargo run
```

*(Later you can move projects to `cargo new` style and add `Cargo.toml` files.)*

## 📈 My learning goal

- Complete 100 exercises.  
- Build 5–10 small CLI tools.  
- Move to **Rust + Solana smart‑contract development** (Anchor, on‑chain programs, dApps).

## 📝 Contributing / feedback

This is a **learning repo**; I’m very open to feedback on:
- Better idiomatic Rust.  
- Improvements in structure, comments, or design.

Feel free to open an issue or PR with suggestions.

## 📄 License

This repository is released under the MIT License.  
See `LICENSE` for details.
