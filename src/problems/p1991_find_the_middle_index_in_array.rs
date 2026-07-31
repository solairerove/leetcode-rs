// time O(n), space O(1)
pub fn find_middle_index(nums: Vec<i32>) -> i32 {
    let sum: i32 = nums.iter().sum();
    let mut left_sum: i32 = 0;
    for i in 0..nums.len() {
        let right_sum = sum - left_sum - nums[i];
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
        assert_eq!(find_middle_index(vec![2, 3, -1, 8, 4]), 3);
    }

    #[test]
    fn no_middle_index() {
        assert_eq!(find_middle_index(vec![1, -1, 4]), 2);
    }

    #[test]
    fn no_middle_index_two_elements() {
        assert_eq!(find_middle_index(vec![2, 5]), -1);
    }

    #[test]
    fn single_element() {
        assert_eq!(find_middle_index(vec![5]), 0);
    }

    #[test]
    fn middle_at_start() {
        assert_eq!(find_middle_index(vec![0, 0, 0]), 0);
    }
}
