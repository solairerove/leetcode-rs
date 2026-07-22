// time O(n * m), space O(m)
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let first = strs[0].as_bytes();
    for i in 0..first.len() {
        for j in 1..strs.len() {
            let sb = strs[j].as_bytes();
            if i == sb.len() || first[i] != sb[i] {
                return String::from_utf8((&first[..i]).to_vec()).unwrap();
            }
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
}
