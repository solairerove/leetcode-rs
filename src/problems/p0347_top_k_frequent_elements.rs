use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

// time O(n log k), space O(n)
pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let k = k as usize;
    let mut count = HashMap::new();
    for &num in &nums {
        *count.entry(num).or_insert(0i32) += 1;
    }

    let mut heap = BinaryHeap::new();
    for (&num, &freq) in &count {
        heap.push(Reverse((freq, num)));
        if heap.len() > k {
            heap.pop();
        }
    }

    heap.into_iter().map(|Reverse((_, num))| num).collect()
}

// TBD: quickselect with Dutch national flag partition

// time O(n + span log k), space O(span + k)
pub fn top_k_frequent_bucket_heap(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let Some((min, max)) = min_max(&nums) else {
        return nums;
    };

    let span = (max as i64 - min as i64 + 1) as usize;
    let mut cnt: Vec<u32> = vec![0u32; span];
    for &num in &nums {
        cnt[(num - min) as usize] += 1;
    }

    let top_k = top_k_heap(&cnt, k as usize);

    top_k.into_iter().map(|offset| min + offset as i32).collect()
}

fn top_k_heap(cnt: &[u32], k: usize) -> Vec<usize> {
    if k == 0 {
        return vec![];
    }

    let mut heap = BinaryHeap::with_capacity(k);

    for (index, &count) in cnt.iter().enumerate() {
        if count == 0 {
            continue;
        }

        if heap.len() < k {
            heap.push((Reverse(count), index));
        } else if let Some(&(Reverse(min_count), _)) = heap.peek() {
            if count > min_count {
                heap.pop();
                heap.push((Reverse(count), index));
            }
        }
    }

    let mut result: Vec<(u32, usize)> = heap
        .into_iter()
        .map(|(Reverse(count), index)| (count, index))
        .collect();

    result.sort_unstable_by(|a, b| b.0.cmp(&a.0));

    result.into_iter().map(|(_, index)| index).collect()
}

fn min_max(v: &[i32]) -> Option<(i32, i32)> {
    let mut it = v.iter().copied();
    let first = it.next()?;
    Some(it.fold((first, first), |(lo, hi), x| (lo.min(x), hi.max(x))))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn as_set(v: Vec<i32>) -> HashSet<i32> {
        v.into_iter().collect()
    }

    #[test]
    fn example_case() {
        let res = top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2);
        assert_eq!(as_set(res), as_set(vec![1, 2]));
    }

    #[test]
    fn single_element() {
        assert_eq!(top_k_frequent(vec![1], 1), vec![1]);
    }

    #[test]
    fn k_equals_two() {
        let res = top_k_frequent(vec![4, 1, -1, 2, -1, 2, 3], 2);
        assert_eq!(as_set(res), as_set(vec![-1, 2]));
    }

    #[test]
    fn negative_numbers() {
        let res = top_k_frequent(vec![-1, -1, -2, -2, -2, -3], 2);
        assert_eq!(as_set(res), as_set(vec![-2, -1]));
    }

    #[test]
    fn all_unique() {
        let res = top_k_frequent(vec![5, 3, 1, 9], 4);
        assert_eq!(as_set(res), as_set(vec![5, 3, 1, 9]));
    }

    #[test]
    fn bucket_heap_example_case() {
        let res = top_k_frequent_bucket_heap(vec![1, 1, 1, 2, 2, 3], 2);
        assert_eq!(as_set(res), as_set(vec![1, 2]));
    }

    #[test]
    fn bucket_heap_single_element() {
        assert_eq!(top_k_frequent_bucket_heap(vec![1], 1), vec![1]);
    }

    #[test]
    fn bucket_heap_k_equals_two() {
        let res = top_k_frequent_bucket_heap(vec![4, 1, -1, 2, -1, 2, 3], 2);
        assert_eq!(as_set(res), as_set(vec![-1, 2]));
    }

    #[test]
    fn bucket_heap_negative_numbers() {
        let res = top_k_frequent_bucket_heap(vec![-1, -1, -2, -2, -2, -3], 2);
        assert_eq!(as_set(res), as_set(vec![-2, -1]));
    }

    #[test]
    fn bucket_heap_all_unique() {
        let res = top_k_frequent_bucket_heap(vec![5, 3, 1, 9], 4);
        assert_eq!(as_set(res), as_set(vec![5, 3, 1, 9]));
    }
}
