# Rust Learning Path

## 1. Fundamentals

### 1.1 Type Aliases
- **File:** [1_fundamentals](src/bin/1_fundamentals.rs#L1-L5)
- **Topics:** Custom type aliases, semantic type definitions
- **Example:** `type Age = u32;`

### 1.2 Vectors & Collections
- **File:** [1_fundamentals](src/bin/1_fundamentals.rs#L8-L9)
- **Topics:** Creating vectors, accessing elements, iteration
- **Example:** `vec![1, 2, 3, 4, 5]`

### 1.3 Pattern Matching & Destructuring
- **File:** [1_fundamentals](src/bin/1_fundamentals.rs#L10-L16)
- **Topics:** Match expressions, array/vector destructuring, rest patterns (`..`)
- **Example:** `match &numbers[..]` with `[first, rest @ ..]`

---

## 2. Collections

### 2.1 HashMaps
- **File:** [hashmaps](src/bin/hashmaps.rs)
- **Topics:** Key-value pairs, insertion, lookup, mutable operations
- **Example:** `HashMap::new()`, `insert()`, `get()`

---

## 3. Ownership & References

### 3.1 Borrowing & Slices
- **Topics:** Slice syntax `&[..]`, immutable borrows

### 3.2 Mutable References
- **Topics:** Mutable borrows, multiple references

---

## 4. Functions & Control Flow

### 4.1 Function Definitions
- **Topics:** Parameters, return types, type annotations

### 4.2 Closures
- **Topics:** Anonymous functions, captures, move semantics

---

## 5. Structs & Enums

### 5.1 Struct Basics
- **Topics:** Defining structs, methods, associated functions

### 5.2 Enums
- **Topics:** Enum variants, pattern matching

---

## Run Examples

To run a specific file:
```bash
cargo run --bin 1_fundamentals
```

To auto-run on save:
```bash
cargo watch -x "run --bin 1_fundamentals"
```
