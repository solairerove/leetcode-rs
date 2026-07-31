#[allow(dead_code)]
struct MyHashMap {
    buckets: Vec<Vec<(i32, i32)>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(dead_code)]
impl MyHashMap {
    // time O(1), space O(n)
    pub fn new() -> Self {
        Self {
            buckets: vec![Vec::new(); 1000],
        }
    }

    fn hash(key: i32) -> usize {
        key as usize % 1000
    }

    // time O(k), space O(1)
    pub fn put(&mut self, key: i32, value: i32) {
        let idx = Self::hash(key);
        for pair in self.buckets[idx].iter_mut() {
            if pair.0 == key {
                pair.1 = value;
                return;
            }
        }
        self.buckets[idx].push((key, value));
    }

    // time O(k), space O(1)
    pub fn get(&self, key: i32) -> i32 {
        let idx = Self::hash(key);
        for pair in &self.buckets[idx] {
            if pair.0 == key {
                return pair.1;
            }
        }
        -1
    }

    // time O(k), space O(1)
    pub fn remove(&mut self, key: i32) {
        let idx = Self::hash(key);
        if let Some(pos) = self.buckets[idx].iter().position(|p| p.0 == key) {
            self.buckets[idx].remove(pos);
        }
    }
}

/**
 * Your MyHashMap object will be instantiated and called as such:
 * let obj = MyHashMap::new();
 * obj.put(key, value);
 * let ret_2: i32 = obj.get(key);
 * obj.remove(key);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_put_get() {
        let mut map = MyHashMap::new();
        map.put(1, 1);
        map.put(2, 2);
        assert_eq!(map.get(1), 1);
        assert_eq!(map.get(3), -1);
    }

    #[test]
    fn test_put_overwrite() {
        let mut map = MyHashMap::new();
        map.put(2, 1);
        assert_eq!(map.get(2), 1);
        map.put(2, 1);
        assert_eq!(map.get(2), 1);
    }

    #[test]
    fn test_remove() {
        let mut map = MyHashMap::new();
        map.put(1, 1);
        map.remove(1);
        assert_eq!(map.get(1), -1);
    }

    #[test]
    fn test_remove_nonexistent_key() {
        let mut map = MyHashMap::new();
        map.remove(5);
        assert_eq!(map.get(5), -1);
    }

    #[test]
    fn test_hash_collision_bucket() {
        let mut map = MyHashMap::new();
        map.put(1, 10);
        map.put(1001, 20);
        assert_eq!(map.get(1), 10);
        assert_eq!(map.get(1001), 20);
        map.remove(1);
        assert_eq!(map.get(1), -1);
        assert_eq!(map.get(1001), 20);
    }

    #[test]
    fn test_zero_key() {
        let mut map = MyHashMap::new();
        assert_eq!(map.get(0), -1);
        map.put(0, 5);
        assert_eq!(map.get(0), 5);
        map.remove(0);
        assert_eq!(map.get(0), -1);
    }
}
