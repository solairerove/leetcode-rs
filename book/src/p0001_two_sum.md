# 1. Two Sum

Given array of integers, return indices of two numbers that add to target.

**Approach:** hashmap of value to index, single pass, complement lookup.

```rust
{{#include ../../src/problems/p0001_two_sum.rs:3:16}}
```

Time O(n), space O(n).
