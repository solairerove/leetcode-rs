// time O(n log n), space O(n)
pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
    let mut scratch = nums.clone();
    merge_sort(&mut nums, &mut scratch);
    nums
}

fn merge_sort(dst: &mut [i32], scratch: &mut [i32]) {
    if dst.len() <= 1 {
        return;
    }

    let mid = dst.len() / 2;
    let (dst_left, dst_right) = dst.split_at_mut(mid);
    let (buf_left, buf_right) = scratch.split_at_mut(mid);

    merge_sort(buf_left, dst_left);
    merge_sort(buf_right, dst_right);

    merge(buf_left, buf_right, dst);
}

fn merge(left: &[i32], right: &[i32], out: &mut [i32]) {
    let (mut i, mut j, mut k) = (0, 0, 0);
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            out[k] = left[i];
            i += 1;
        } else {
            out[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    out[k..].copy_from_slice(if i < left.len() {
        &left[i..]
    } else {
        &right[j..]
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsorted() {
        assert_eq!(sort_array(vec![5, 2, 3, 1]), vec![1, 2, 3, 5]);
    }

    #[test]
    fn test_with_duplicates() {
        assert_eq!(sort_array(vec![5, 1, 1, 2, 0, 0]), vec![0, 0, 1, 1, 2, 5]);
    }

    #[test]
    fn test_already_sorted() {
        assert_eq!(sort_array(vec![1, 2, 3]), vec![1, 2, 3]);
    }

    #[test]
    fn test_reverse_sorted() {
        assert_eq!(sort_array(vec![3, 2, 1]), vec![1, 2, 3]);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(sort_array(vec![1]), vec![1]);
    }

    #[test]
    fn test_negative_numbers() {
        assert_eq!(sort_array(vec![-3, -1, -2]), vec![-3, -2, -1]);
    }
}
