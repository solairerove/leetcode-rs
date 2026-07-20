// time O(n), space O(n)
pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
    let original_size = nums.len();
    let size = original_size * 2;
    let mut ans: Vec<i32> = Vec::with_capacity(size as usize);
    for i in 0..size {
        ans.push(nums[i % original_size]);
    }

    return ans;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doubles_sequence() {
        assert_eq!(
            get_concatenation(vec![1, 2, 1]),
            vec![1, 2, 1, 1, 2, 1]
        );
    }

    #[test]
    fn handles_single_element() {
        assert_eq!(get_concatenation(vec![1, 3, 2, 1]), vec![1, 3, 2, 1, 1, 3, 2, 1]);
    }

    #[test]
    fn handles_single() {
        assert_eq!(get_concatenation(vec![7]), vec![7, 7]);
    }
}
