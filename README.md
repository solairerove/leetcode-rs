# leetcode-rs

1. Every value has exactly one owner at a time.
2. When the owner goes out of scope, the value is dropped (freed) — no GC needed.
3. You can either have one mutable reference OR any number of read-only references to a value — never both at once.