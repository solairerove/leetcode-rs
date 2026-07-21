# 1929. Concatenation of Array

Given array `nums`, return array `ans` of length `2n` where `ans[i] == nums[i]` and `ans[i + n] == nums[i]` for `0 <= i < n`.

**Approach:** allocate result of double size upfront, fill by indexing source with modulo.

```rust
{{#include ../../src/problems/p1929_concatenation_of_array.rs:1:11}}
```
