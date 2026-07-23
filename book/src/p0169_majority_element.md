# 169. Majority Element

Given an array `nums` of size `n`, return the majority element.

The majority element is the element that appears more than `⌊n / 2⌋` times. You may assume that the majority element always exists in the array.

Example 1:

```
Input: nums = [3,2,3]
Output: 3
```

Example 2:

```
Input: nums = [2,2,1,1,1,2,2]
Output: 2
```

**HashMap counting:**

```rust
{{#include ../../src/problems/p0169_majority_element.rs:4:19}}
```

**Sort and take the middle:**

```rust
{{#include ../../src/problems/p0169_majority_element.rs:22:27}}
```

**Boyer-Moore voting:**

```rust
{{#include ../../src/problems/p0169_majority_element.rs:29:41}}
```

**Randomization:**

```rust
{{#include ../../src/problems/p0169_majority_element.rs:43:53}}
```
