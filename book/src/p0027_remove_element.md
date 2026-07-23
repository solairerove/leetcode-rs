# 27. Remove Element

Given an integer array `nums` and an integer `val`, remove all occurrences of `val` in `nums` in-place. The order of the elements may be changed. Then return the number of elements in `nums` which are not equal to `val`.

Consider the number of elements in `nums` which are not equal to `val` be `k`, to get accepted, you need to do the following things:

Change the array `nums` such that the first `k` elements of `nums` contain the elements which are not equal to `val`. The remaining elements of `nums` are not important as well as the size of `nums`.
Return `k`.

**Example 1:**

Input: `nums = [3,2,2,3]`, `val = 3`

Output: `k = 2`, `nums = [2,2,_,_]`

**Example 2:**

Input: `nums = [0,1,2,2,3,0,4,2]`, `val = 2`

Output: `k = 5`, `nums = [0,1,3,0,4,_,_,_]`

**Custom Judge:**

The judge will test your solution with the following code:

```java
int[] nums = [...]; // Input array
int val = ...; // Value to remove
int[] expectedNums = [...]; // The expected answer with correct length.
                            // It is sorted with no values equaling val.

int k = removeElement(nums, val); // Calls your implementation

assert k == expectedNums.length;
sort(nums, 0, k); // Sort the first k elements of nums
for (int i = 0; i < k; i++) {
    assert nums[i] == expectedNums[i];
}
```

**Swap-with-last approach:**

`i` scans forward from the start, `n` tracks the current logical end of the array. When `nums[i] == val`, shrink `n` by one and overwrite `nums[i]` with whatever sits at the new last index `nums[n]`, then re-check the same `i` (the swapped-in value hasn't been checked yet). When `nums[i] != val`, it's already in the "keep" region, so just advance `i`. Elements equal to `val` end up pushed past index `n`, and the loop only ever touches each index once as either the scanning pointer or the source of a swap, giving O(k) time (k = `nums.len()`) and O(1) extra space.

```rust
{{#include ../../src/problems/p0027_remove_element.rs:2:15}}
```
