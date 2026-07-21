use std::collections::HashMap;

// time O(n), space O(n)
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut num_to_idx: HashMap<i32, i32> = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        let guess = target - num;
        if let Some(&x) = num_to_idx.get(&guess) {
            return vec![x, i as i32];
        }
        num_to_idx.insert(num, i as i32);
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_pair() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn finds_pair_not_adjacent() {
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn finds_duplicate_pair() {
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }

    #[test]
    fn returns_empty_when_no_pair() {
        assert_eq!(two_sum(vec![1, 2, 3], 100), Vec::<i32>::new());
    }
}
