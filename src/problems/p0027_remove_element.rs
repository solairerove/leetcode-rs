// time O(k), space O(1)
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut i = 0;
    let mut n = nums.len();
    while i < n {
        if nums[i] == val {
            n -= 1;
            nums[i] = nums[n];
        } else {
            i += 1;
        }
    }

    n as i32
}

pub fn remove_element_two_pointer_swap(nums: &mut Vec<i32>, val: i32) -> i32 {
    let n = nums.len();
    if n == 0 {
        return 0;
    }

    if n == 1 {
        if nums[0] == val {
            return 0;
        } else {
            return 1;
        }
    }

    let mut left_most_equal_to_val_idx = 0;
    let mut right_most_not_equal_to_val_idx = n - 1;
    while left_most_equal_to_val_idx <= right_most_not_equal_to_val_idx {
        while nums[left_most_equal_to_val_idx] != val
            && left_most_equal_to_val_idx < n - 1
            && left_most_equal_to_val_idx < right_most_not_equal_to_val_idx
        {
            left_most_equal_to_val_idx += 1;
        }

        while nums[right_most_not_equal_to_val_idx] == val
            && right_most_not_equal_to_val_idx > 0
            && left_most_equal_to_val_idx <= right_most_not_equal_to_val_idx
        {
            right_most_not_equal_to_val_idx -= 1;
        }

        print!(
            "left: {}, right: {} \n",
            left_most_equal_to_val_idx, right_most_not_equal_to_val_idx
        );
        if left_most_equal_to_val_idx < right_most_not_equal_to_val_idx {
            print!(
                "swap {}, i {} j {}\n",
                nums[left_most_equal_to_val_idx],
                left_most_equal_to_val_idx,
                right_most_not_equal_to_val_idx
            );
            swap(
                nums,
                left_most_equal_to_val_idx,
                right_most_not_equal_to_val_idx,
            );
        } else {
            break;
        }
    }

    for n in nums {
        print!("{} ", n);
    }

    if left_most_equal_to_val_idx == 0 {
        return 0;
    }

    (right_most_not_equal_to_val_idx + 1) as i32
}

fn swap<T: Copy>(arr: &mut Vec<T>, i: usize, j: usize) {
    let temp = arr[i];
    arr[i] = arr[j];
    arr[j] = temp;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(mut nums: Vec<i32>, val: i32, mut expected: Vec<i32>) {
        let k = remove_element(&mut nums, val);
        assert_eq!(k, expected.len() as i32);
        let mut actual = nums[..k as usize].to_vec();
        actual.sort();
        expected.sort();
        assert_eq!(actual, expected);
    }

    fn check_two_pointer_swap(mut nums: Vec<i32>, val: i32, mut expected: Vec<i32>) {
        let k = remove_element_two_pointer_swap(&mut nums, val);
        assert_eq!(k, expected.len() as i32);
        let mut actual = nums[..k as usize].to_vec();
        actual.sort();
        expected.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    fn removes_val_with_duplicates_at_ends() {
        check(vec![3, 2, 2, 3], 3, vec![2, 2]);
    }

    #[test]
    fn removes_val_scattered() {
        check(vec![0, 1, 2, 2, 3, 0, 4, 2], 2, vec![0, 1, 3, 0, 4]);
    }

    #[test]
    fn no_occurrences_of_val() {
        check(vec![4, 5], 4, vec![5]);
    }

    #[test]
    fn all_elements_equal_val() {
        check(vec![3, 3], 3, vec![]);
    }

    #[test]
    fn two_pointer_swap_removes_val_with_duplicates_at_ends() {
        check_two_pointer_swap(vec![3, 2, 2, 3], 3, vec![2, 2]);
    }

    #[test]
    fn two_pointer_swap_removes_val_scattered() {
        check_two_pointer_swap(vec![0, 1, 2, 2, 3, 0, 4, 2], 2, vec![0, 1, 3, 0, 4]);
    }

    #[test]
    fn two_pointer_swap_no_occurrences_of_val() {
        check_two_pointer_swap(vec![4, 5], 4, vec![5]);
    }

    #[test]
    fn two_pointer_swap_all_elements_equal_val() {
        check_two_pointer_swap(vec![3, 3], 3, vec![]);
    }
}
