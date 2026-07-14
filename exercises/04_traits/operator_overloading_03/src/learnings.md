# Operator Overloading in Rust: A Deep Dive with PartialEq

This document chronicles our journey through operator overloading in Rust. We started with the high-level theory, moved to a practical `Add` example, and then debugged a broken `PartialEq` implementation for a `Ticket` struct—fixing critical mistakes along the way.

---

## 1. What is Operator Overloading?

In Rust, **operator overloading** means giving custom behavior to standard symbols like `+`, `-`, `*`, `==`, and `>` for your own structs or enums.

Unlike C++ or Python where you can overload anything freely, Rust **strictly enforces** that every operator is backed by a specific trait in the standard library.

### The Golden Rule: One Operator = One Trait

Rust maps every operator to a trait in the `std::ops` or `std::cmp` module.

| Operator | Trait | Method you implement |
| :--- | :--- | :--- |
| `+` (Addition) | `std::ops::Add` | `fn add(self, rhs: Rhs) -> Self::Output` |
| `-` (Subtraction) | `std::ops::Sub` | `fn sub(self, rhs: Rhs) -> Self::Output` |
| `*` (Multiplication) | `std::ops::Mul` | `fn mul(self, rhs: Rhs) -> Self::Output` |
| `==` and `!=` (Equality) | `std::cmp::PartialEq` | `fn eq(&self, other: &Rhs) -> bool` |
| `<`, `>`, `<=`, `>=` | `std::cmp::PartialOrd` | `fn partial_cmp(&self, other: &Rhs) -> Option<Ordering>` |

---

## 2. Example 1: Overloading `+` for a `Point` Struct

Let's see a simple, working example of overloading the `+` operator.

```rust
use std::ops::Add;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// Overload the `+` operator for Point + Point
impl Add for Point {
    // Associated type: defines what type the addition returns
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2; // Now this works!

    println!("{:?}", p3); // Prints: Point { x: 4, y: 6 }
}

## Key Observations:
---
Add takes ownership of self and other (they are moved). This is fine for small structs, but not ideal for large ones.

You can also implement Add<&Point> for Point to borrow instead.

## 3. The Challenge: Implementing PartialEq for a Ticket
We were tasked with implementing the PartialEq trait for a Ticket struct so that we could compare two tickets for equality.

## The Broken Code (Our Starting Point)
Here is the code that was initially written—it contains several critical mistakes:

rust
use std::cmp::PartialEq;

struct Ticket {
    title: String,
    description: String,
    status: String,
}

// WRONG: Redefining the trait instead of implementing it!
trait PartialEq {
    fn ticket(&self)->Ticket{
      self.title = title,
      self.description = description,
      self.status = status,
    }
}

// WRONG: This tries to implement our custom (broken) trait, not the standard one.
impl PartialEq for Ticket {

}
## 4. The Mistakes We Fixed
🔴 Mistake #1: Redefining the Standard Trait
Problem: We used use std::cmp::PartialEq; to import the trait, but then we defined our own custom trait with the same name. This shadowed the standard one.

Fix: Delete the custom trait definition entirely. Rust already provides PartialEq. You just need to impl it.

🔴 Mistake #2: Wrong Method Name and Signature
Problem: We wrote fn ticket(&self) -> Ticket, but the standard PartialEq requires fn eq(&self, other: &Self) -> bool.

Fix: Implement fn eq(...) with the correct signature.

🔴 Mistake #3: Using = (Assignment) instead of == (Comparison)
Problem: We wrote self.title = title, which tries to assign a value to self.title.

Fix: We need self.title == other.title to compare the two titles.

🔴 Mistake #4: Missing & before other
Problem: We wrote other: Self, which takes ownership of the other ticket (moves it into the function, destroying it).

Fix: Use other: &Self to borrow the other ticket.

🔴 Mistake #5: Forgetting the && (Logical AND)
Problem: We had commas between conditions, which is invalid syntax.

Fix: Use && to combine all three comparisons.

## 5. The Correct Implementation
After fixing all the mistakes, here is the final, polished code:
---

```rust
use std::cmp::PartialEq;

struct Ticket {
    title: String,
    description: String,
    status: String,
}

// Correctly implement the standard PartialEq trait
impl PartialEq for Ticket {
    // &self  -> Borrow the left ticket (don't destroy it)
    // &Self  -> Borrow the right ticket (don't destroy it)
    // -> bool -> Returns true if all fields match
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
            && self.description == other.description
            && self.status == other.status
    }
}

---
## 6. Deep Dive: Why &self and &Self?
This is the most important part of the eq method signature.

### The Golden Rule of eq
Comparing two things should not change or destroy them. Therefore, both sides must be borrowed.

&self: Borrow the current ticket (read-only). If we took self (without &), the ticket would be moved and destroyed.

&Self: Borrow the other ticket (read-only). If we took other: Self (without &), the other ticket would be moved and destroyed.

###What happens if we take ownership?
---

```rust
// BAD: Takes ownership of 'other'
fn eq(&self, other: Ticket) -> bool {
    // ...
}

let ticket1 = Ticket { ... };
let ticket2 = Ticket { ... };

let result = ticket1.eq(ticket2); // ticket2 is MOVED here!
println!("{:?}", ticket2); // ❌ ERROR: ticket2 was destroyed!
What happens with & (Borrowing)?
rust
// GOOD: Borrows 'other'
fn eq(&self, other: &Ticket) -> bool {
    // ...
}

let ticket1 = Ticket { ... };
let ticket2 = Ticket { ... };

let result = ticket1.eq(&ticket2); // Pass a reference!
println!("{:?}", ticket2); // ✅ Works perfectly!

---
## 7. Deep Dive: Why && (Logical AND)?
&& is the short-circuiting logical AND operator. It combines multiple boolean conditions.

Without &&:
You would have to write ugly, nested if statements:
---

```rust
fn eq(&self, other: &Self) -> bool {
    if self.title == other.title {
        if self.description == other.description {
            if self.status == other.status {
                return true;
            }
        }
    }
    false
}
With &&:
It reads like plain English and is much cleaner:

```rust
self.title == other.title
    && self.description == other.description
    && self.status == other.status
👉 "Return true if the titles match AND the descriptions match AND the statuses match."

---
## Short-Circuiting Behavior
&& evaluates from left to right. If the first condition is false, it does not even check the rest because the whole expression can never be true.
---

```rust
self.title == other.title  // If this is false, Rust STOPS here!
    && self.description == other.description // Skipped if title didn't match
    && self.status == other.status // Skipped if title didn't match



