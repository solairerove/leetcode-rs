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
}
