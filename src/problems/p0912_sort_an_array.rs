use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};

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

// time O(n log n) expected, space O(log n) expected
pub fn sort_array_quicksort(mut nums: Vec<i32>) -> Vec<i32> {
    let mut rng = Rng::new();
    quicksort(&mut nums, &mut rng);
    nums
}

fn quicksort(v: &mut [i32], rng: &mut Rng) {
    if v.len() <= 1 {
        return;
    }

    let pivot = v[rng.below(v.len())];
    let (lt, gt) = partition(v, pivot);

    let (less, rest) = v.split_at_mut(lt);
    let (_equal, greater) = rest.split_at_mut(gt - lt);

    quicksort(less, rng);
    quicksort(greater, rng);
}

fn partition(v: &mut [i32], pivot: i32) -> (usize, usize) {
    let (mut lt, mut i, mut gt) = (0, 0, v.len());
    while i < gt {
        if v[i] < pivot {
            v.swap(lt, i);
            lt += 1;
            i += 1;
        } else if v[i] > pivot {
            gt -= 1;
            v.swap(i, gt);
        } else {
            i += 1;
        }
    }

    (lt, gt)
}

struct Rng(u64);

impl Rng {
    fn new() -> Self {
        Rng(RandomState::new().build_hasher().finish() | 1)
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    fn below(&mut self, n: usize) -> usize {
        ((self.next_u64() as u128 * n as u128) >> 64) as usize
    }
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

    #[test]
    fn test_unsorted_quicksort() {
        assert_eq!(sort_array_quicksort(vec![5, 2, 3, 1]), vec![1, 2, 3, 5]);
    }

    #[test]
    fn test_with_duplicates_quicksort() {
        assert_eq!(
            sort_array_quicksort(vec![5, 1, 1, 2, 0, 0]),
            vec![0, 0, 1, 1, 2, 5]
        );
    }

    #[test]
    fn test_already_sorted_quicksort() {
        assert_eq!(sort_array_quicksort(vec![1, 2, 3]), vec![1, 2, 3]);
    }

    #[test]
    fn test_reverse_sorted_quicksort() {
        assert_eq!(sort_array_quicksort(vec![3, 2, 1]), vec![1, 2, 3]);
    }

    #[test]
    fn test_single_element_quicksort() {
        assert_eq!(sort_array_quicksort(vec![1]), vec![1]);
    }

    #[test]
    fn test_negative_numbers_quicksort() {
        assert_eq!(sort_array_quicksort(vec![-3, -1, -2]), vec![-3, -2, -1]);
    }
}
