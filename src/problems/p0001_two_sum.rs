use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut num_to_idx: HashMap<i32, usize> = HashMap::new();
    for (i, &e) in nums.iter().enumerate() {
        let guess = target - e;
        if let Some(&index) = num_to_idx.get(&guess) {
            return vec![index as i32, i as i32];
        }

        num_to_idx.insert(e, i);
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
