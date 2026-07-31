// time O(n), space O(1)
pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let total: i32 = nums.iter().sum();
    let mut left_sum = 0;
    for i in 0..nums.len() {
        let right_sum = total - left_sum - nums[i];
        if left_sum == right_sum {
            return i as i32;
        }
        left_sum += nums[i];
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_case() {
        assert_eq!(pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
    }

    #[test]
    fn no_pivot() {
        assert_eq!(pivot_index(vec![1, 2, 3]), -1);
    }

    #[test]
    fn pivot_at_start() {
        assert_eq!(pivot_index(vec![2, 1, -1]), 0);
    }

    #[test]
    fn single_element() {
        assert_eq!(pivot_index(vec![5]), 0);
    }

    #[test]
    fn all_zeros() {
        assert_eq!(pivot_index(vec![0, 0, 0, 0]), 0);
    }
}
