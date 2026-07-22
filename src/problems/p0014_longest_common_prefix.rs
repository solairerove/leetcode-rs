// time O(n * m), space O(m)
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let first = strs[0].as_bytes();
    for i in 0..first.len() {
        for j in 1..strs.len() {
            let sb = strs[j].as_bytes();
            if i == sb.len() || first[i] != sb[i] {
                return strs[0][..i].to_string();
            }
        }
    }

    strs[0].clone()
}

// time O(n log n * m), space O(1) excl. sort
pub fn longest_common_prefix_sorted(strs: Vec<String>) -> String {
    if strs.len() == 1 {
        return strs[0].clone();
    }

    let mut strs = strs;
    strs.sort();

    let first = strs[0].as_bytes();
    let last = strs[strs.len() - 1].as_bytes();
    let n = first.len().min(last.len());
    for i in 0..n {
        if first[i] != last[i] {
            return strs[0][..i].to_string();
        }
    }

    strs[0].clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_common_prefix() {
        assert_eq!(
            longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            "fl"
        );
    }

    #[test]
    fn no_common_prefix() {
        assert_eq!(
            longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ]),
            ""
        );
    }

    #[test]
    fn single_string() {
        assert_eq!(longest_common_prefix(vec!["alone".to_string()]), "alone");
    }

    #[test]
    fn identical_strings() {
        assert_eq!(
            longest_common_prefix(vec!["test".to_string(), "test".to_string()]),
            "test"
        );
    }

    #[test]
    fn one_is_prefix_of_other() {
        assert_eq!(
            longest_common_prefix(vec!["flow".to_string(), "flower".to_string()]),
            "flow"
        );
    }

    #[test]
    fn sorted_finds_common_prefix() {
        assert_eq!(
            longest_common_prefix_sorted(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            "fl"
        );
    }

    #[test]
    fn sorted_no_common_prefix() {
        assert_eq!(
            longest_common_prefix_sorted(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ]),
            ""
        );
    }

    #[test]
    fn sorted_single_string() {
        assert_eq!(
            longest_common_prefix_sorted(vec!["alone".to_string()]),
            "alone"
        );
    }

    #[test]
    fn sorted_identical_strings() {
        assert_eq!(
            longest_common_prefix_sorted(vec!["test".to_string(), "test".to_string()]),
            "test"
        );
    }

    #[test]
    fn sorted_one_is_prefix_of_other() {
        assert_eq!(
            longest_common_prefix_sorted(vec!["flow".to_string(), "flower".to_string()]),
            "flow"
        );
    }
}
