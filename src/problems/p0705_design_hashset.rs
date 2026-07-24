struct MyHashSet {
    buckets: Vec<Vec<i32>>,
}

impl MyHashSet {
    // time O(1), space O(n)
    pub fn new() -> Self {
        Self {
            buckets: vec![Vec::new(); 10_000],
        }
    }

    fn hash(key: i32) -> usize {
        key as usize % 10_000
    }

    // time O(k), space O(1)
    pub fn add(&mut self, key: i32) {
        let idx = Self::hash(key);
        if !self.buckets[idx].contains(&key) {
            self.buckets[idx].push(key);
        }
    }

    // time O(k), space O(1)
    pub fn remove(&mut self, key: i32) {
        let idx = Self::hash(key);
        if let Some(i) = self.buckets[idx].iter().position(|&x| x == key) {
            self.buckets[idx].remove(i);
        }
    }

    // time O(k), space O(1)
    pub fn contains(&self, key: i32) -> bool {
        let idx = Self::hash(key);

        self.buckets[idx].contains(&key)
    }
}

/**
 * Your MyHashSet object will be instantiated and called as such:
 * let obj = MyHashSet::new();
 * obj.add(key);
 * obj.remove(key);
 * let ret_3: bool = obj.contains(key);
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_contains() {
        let mut set = MyHashSet::new();
        set.add(1);
        set.add(2);
        assert!(set.contains(1));
        assert!(!set.contains(3));
    }

    #[test]
    fn test_add_remove_contains() {
        let mut set = MyHashSet::new();
        set.add(2);
        assert!(set.contains(2));
        set.remove(2);
        assert!(!set.contains(2));
    }

    #[test]
    fn test_remove_nonexistent_key() {
        let mut set = MyHashSet::new();
        set.remove(5);
        assert!(!set.contains(5));
    }

    #[test]
    fn test_add_duplicate_key() {
        let mut set = MyHashSet::new();
        set.add(1);
        set.add(1);
        assert!(set.contains(1));
        set.remove(1);
        assert!(!set.contains(1));
    }

    #[test]
    fn test_hash_collision_bucket() {
        let mut set = MyHashSet::new();
        set.add(1);
        set.add(10_001);
        assert!(set.contains(1));
        assert!(set.contains(10_001));
        set.remove(1);
        assert!(!set.contains(1));
        assert!(set.contains(10_001));
    }

    #[test]
    fn test_zero_key() {
        let mut set = MyHashSet::new();
        assert!(!set.contains(0));
        set.add(0);
        assert!(set.contains(0));
        set.remove(0);
        assert!(!set.contains(0));
    }
}
