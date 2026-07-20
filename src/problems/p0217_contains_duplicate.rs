use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut unique: HashSet<i32> = HashSet::with_capacity(nums.len());
    for num in nums {
        if !unique.insert(num) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_duplicate() {
        assert!(contains_duplicate(vec![1, 2, 3, 1]));
    }

    #[test]
    fn all_unique() {
        assert!(!contains_duplicate(vec![1, 2, 3, 4]));
    }

    #[test]
    fn handles_empty() {
        assert!(!contains_duplicate(vec![]));
    }

    #[test]
    fn handles_single() {
        assert!(!contains_duplicate(vec![1]));
    }
}
