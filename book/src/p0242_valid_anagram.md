# 242. Valid Anagram

Check if `t` is an anagram of `s`.

**HashMap approach:**

```rust
{{#include ../../src/problems/p0242_valid_anagram.rs:3:21}}
```

**Fixed array approach** (ASCII lowercase only, faster, O(1) space):

```rust
{{#include ../../src/problems/p0242_valid_anagram.rs:23:34}}
```
