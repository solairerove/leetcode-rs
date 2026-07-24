# 705. Design HashSet

Design a HashSet without using any built-in hash table libraries.

Implement `MyHashSet` class:

- `void add(key)` Inserts the value `key` into the HashSet.
- `bool contains(key)` Returns whether the value `key` exists in the HashSet or not.
- `void remove(key)` Removes the value `key` in the HashSet. If `key` does not exist in the HashSet, do nothing.

Example 1:

```
Input
["MyHashSet", "add", "add", "contains", "contains", "add", "contains", "remove", "contains"]
[[], [1], [2], [1], [3], [2], [2], [2], [2]]
Output
[null, null, null, true, false, null, true, null, false]
```

**Bucket of BTreeSets:**

```rust
{{#include ../../src/problems/p0705_design_hashset.rs:3:38}}
```

**Bucket of Vecs (naive linear scan):**

```rust
{{#include ../../src/problems/p0705_design_hashset.rs:48:86}}
```

**Bitset (not viable for arbitrary `i32` — assumes non-negative keys within a fixed upper bound):**

```rust
{{#include ../../src/problems/p0705_design_hashset.rs:88:120}}
```
