# 347. Top K Frequent Elements

Given an integer array `nums` and an integer `k`, return the `k` most frequent elements. The answer may be returned in any order.

Example 1:

```
Input: nums = [1,1,1,2,2,3], k = 2
Output: [1,2]
```

Example 2:

```
Input: nums = [1], k = 1
Output: [1]
```

**Bucket by frequency:**

```rust
{{#include ../../src/problems/p0347_top_k_frequent_elements.rs:5:29}}
```

**Min-heap of size k:**

```rust
{{#include ../../src/problems/p0347_top_k_frequent_elements.rs:33:50}}
```
