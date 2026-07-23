use rand::RngExt;
use std::collections::HashMap;

// time O(n), space O(n)
pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    let mut res = 0;
    let mut max = 0;
    for num in nums {
        let v = cnt.entry(num).or_insert(0);
        *v += 1;
        if *v > max {
            res = num;
            max = *v;
        }
    }

    res
}

// time O(n log n), space O(1)
pub fn majority_element_sort(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    nums.sort();

    nums[nums.len() / 2]
}

// time O(n), space O(1)
pub fn majority_element_boyer_moore(nums: Vec<i32>) -> i32 {
    let mut res = 0;
    let mut cnt = 0;
    for num in nums {
        if cnt == 0 {
            res = num;
        }
        cnt += if num == res { 1 } else { -1 };
    }

    res
}

// time O(n) expected, space O(1)
pub fn majority_element_random(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    loop {
        let guess = nums[rand::rng().random_range(0..n)]; // let guess = nums[rand::random::<usize>() % n]; for leetcode
        let cnt = nums.iter().filter(|&&x| x == guess).count();
        if cnt > n / 2 {
            return guess;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(majority_element(vec![3, 2, 3]), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }

    #[test]
    fn handles_single() {
        assert_eq!(majority_element(vec![1]), 1);
    }

    #[test]
    fn sort_example_1() {
        assert_eq!(majority_element_sort(vec![3, 2, 3]), 3);
    }

    #[test]
    fn sort_example_2() {
        assert_eq!(majority_element_sort(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }

    #[test]
    fn sort_handles_single() {
        assert_eq!(majority_element_sort(vec![1]), 1);
    }

    #[test]
    fn boyer_moore_example_1() {
        assert_eq!(majority_element_boyer_moore(vec![3, 2, 3]), 3);
    }

    #[test]
    fn boyer_moore_example_2() {
        assert_eq!(majority_element_boyer_moore(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }

    #[test]
    fn boyer_moore_handles_single() {
        assert_eq!(majority_element_boyer_moore(vec![1]), 1);
    }

    #[test]
    fn random_example_1() {
        assert_eq!(majority_element_random(vec![3, 2, 3]), 3);
    }

    #[test]
    fn random_example_2() {
        assert_eq!(majority_element_random(vec![2, 2, 1, 1, 1, 2, 2]), 2);
    }

    #[test]
    fn random_handles_single() {
        assert_eq!(majority_element_random(vec![1]), 1);
    }
}
