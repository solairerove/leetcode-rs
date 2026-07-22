# 14. Longest Common Prefix

Find the longest common prefix string amongst an array of strings.

**Vertical scan approach:**

```rust
{{#include ../../src/problems/p0014_longest_common_prefix.rs:2:14}}
```

**Sort approach** (only first/last need comparing after sorting):

```rust
{{#include ../../src/problems/p0014_longest_common_prefix.rs:17:35}}
```
