# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Your role here

User writes own solutions. When given a solution:

- Create new file for it under `src/problems/` (naming/convention rules below).
- Add tests covering it.
- Wire into `src/problems/mod.rs`.
- No suggestions, no assumptions, no rewriting user's logic. Just place it and cover with tests.

## Ownership rules (README.md)

1. Every value has exactly one owner at a time.
2. When the owner goes out of scope, the value is dropped (freed) — no GC needed.
3. You can either have one mutable reference OR any number of read-only references to a value — never both at once.

## Commands

- Build: `cargo build`
- Run all tests: `cargo test`
- Run tests for one problem: `cargo test p0021` (module name prefix matches file name, e.g. `p0021_merge_two_sorted_lists`)
- Lint: `cargo clippy`
- Format: `cargo fmt`

## Architecture

LeetCode solutions in Rust, one file per problem under `src/problems/`.

- `src/lib.rs` — crate root, exposes `common` and `problems` modules.
- `src/problems/mod.rs` — declares each problem module; every new problem file must be added here.
- `src/problems/pNNNN_snake_case_title.rs` — one file per problem, numbered with LeetCode problem ID (zero-padded to 4 digits). Contains the solution function plus a `#[cfg(test)] mod tests` block with unit tests in the same file.
- `src/common/` — shared data structures/helpers reused across problems (e.g. `ListNode` for linked-list problems, with `from_vec`/`to_vec` conversion helpers for building test fixtures). Re-exported via `src/common/mod.rs`.

### Conventions observed in existing solutions

- Solution functions use LeetCode's original signature/argument style (e.g. `Option<Box<ListNode>>` for linked lists), not idiomatic Rust wrappers.
- Strip LeetCode's `impl Solution { ... }` wrapper — solution is a free `pub fn` at module top level, not a method on a `Solution` struct.
- A `// time O(...), space O(...)` complexity comment precedes each solution function.
- Tests use `from_vec`/`to_vec` from `crate::common::list` to convert between `Vec<i32>` and linked-list fixtures.
