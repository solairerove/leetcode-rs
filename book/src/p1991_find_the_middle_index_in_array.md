# 1991. Find the Middle Index in Array

Given array `nums`, return the leftmost middle index where sum of elements to the left equals sum of elements to the right. Return `-1` if none exists.

**Approach:** compute total sum upfront, then walk left to right tracking running left sum; right sum derived as `sum - left_sum - nums[i]`.

```rust
{{#include ../../src/problems/p1991_find_the_middle_index_in_array.rs:1:14}}
```
