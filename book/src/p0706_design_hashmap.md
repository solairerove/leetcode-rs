# 706. Design HashMap

Design a HashMap without using any built-in hash table libraries.

Implement `MyHashMap` class:

- `MyHashMap()` initializes the object with an empty map.
- `void put(key, value)` inserts a `(key, value)` pair into the HashMap. If the `key` already exists, update the corresponding `value`.
- `int get(key)` returns the `value` to which the specified `key` is mapped, or `-1` if this map contains no mapping for the `key`.
- `void remove(key)` removes the `key` and its corresponding `value` if the map contains the mapping for the `key`.

Example 1:

```
Input
["MyHashMap", "put", "put", "get", "get", "put", "get", "remove", "get"]
[[], [1, 1], [2, 2], [1], [3], [2, 1], [2], [2], [2]]
Output
[null, null, null, 1, -1, null, 1, null, -1]
```

**Bucket of Vecs (linear scan for the key):**

```rust
{{#include ../../src/problems/p0706_design_hashmap.rs:1:53}}
```
