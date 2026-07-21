# leetcode-rs

## Book

Solutions are published as a static site via [mdBook](https://rust-lang.github.io/mdBook/), pulling code directly from `src/problems/*.rs` (no copy-pasting into the book — the `.rs` files are the single source of truth).

Install once: `brew install mdbook`

Rebuild after adding/editing a solution:

```
cd book
mdbook build
```

Output lands in `book/book/` (open `book/book/index.html` to view). To add a new problem to the book: create `book/src/pNNNN_title.md` with an `{{#include ../../src/problems/pNNNN_title.rs:START:END}}` block pointing at the relevant line range, and add it to `book/src/SUMMARY.md`.

Preview with live reload while editing: `mdbook serve` (from `book/`).

## Ownership rules

1. Every value has exactly one owner at a time.
2. When the owner goes out of scope, the value is dropped (freed) — no GC needed.
3. You can either have one mutable reference OR any number of read-only references to a value — never both at once.