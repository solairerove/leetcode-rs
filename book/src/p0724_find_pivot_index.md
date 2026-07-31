# 724. Find Pivot Index

Given array `nums`, return the leftmost pivot index where sum of elements to the left equals sum of elements to the right. Return `-1` if none exists.

**Approach:** compute total sum upfront, then walk left to right tracking running left sum; right sum derived as `total - left_sum - nums[i]`.

```rust
{{#include ../../src/problems/p0724_find_pivot_index.rs:1:14}}
```
