// Dutch National Flag algorithm
// time O(n), space O(1)
pub fn sort_colors(nums: &mut Vec<i32>) {
    let mut i: usize = 0;
    let mut l: usize = 0;
    let mut r: usize = nums.len() - 1;
    while i <= r {
        if nums[i] == 0 {
            nums.swap(l, i);
            l += 1;
            i += 1;
        } else if nums[i] == 2 {
            nums.swap(i, r);
            if r == 0 {
                break;
            }
            r -= 1;
        } else {
            i += 1;
        }
    }
}

// time O(n), space O(1)
pub fn sort_colors_bucket(nums: &mut Vec<i32>) {
    let mut cnt = [0; 3];
    for &num in nums.iter() {
        cnt[num as usize] += 1;
    }

    let mut idx: usize = 0;
    for i in 0..3 {
        while cnt[i] > 0 {
            cnt[i] -= 1;
            nums[idx] = i as i32;
            idx += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mixed_colors() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn test_already_sorted() {
        let mut nums = vec![0, 1, 2];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 1, 2]);
    }

    #[test]
    fn test_single_element() {
        let mut nums = vec![1];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![1]);
    }

    #[test]
    fn test_all_same_color() {
        let mut nums = vec![1, 1, 1];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![1, 1, 1]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut nums = vec![2, 1, 0];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 1, 2]);
    }

    #[test]
    fn test_mixed_colors_bucket() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        sort_colors_bucket(&mut nums);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn test_already_sorted_bucket() {
        let mut nums = vec![0, 1, 2];
        sort_colors_bucket(&mut nums);
        assert_eq!(nums, vec![0, 1, 2]);
    }

    #[test]
    fn test_single_element_bucket() {
        let mut nums = vec![1];
        sort_colors_bucket(&mut nums);
        assert_eq!(nums, vec![1]);
    }

    #[test]
    fn test_all_same_color_bucket() {
        let mut nums = vec![1, 1, 1];
        sort_colors_bucket(&mut nums);
        assert_eq!(nums, vec![1, 1, 1]);
    }

    #[test]
    fn test_reverse_sorted_bucket() {
        let mut nums = vec![2, 1, 0];
        sort_colors_bucket(&mut nums);
        assert_eq!(nums, vec![0, 1, 2]);
    }
}
