use std::collections::HashMap;

// time O(n * k), space O(n * k)
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<Vec<u8>, Vec<String>> = HashMap::with_capacity(strs.len());

    for s in strs {
        let mut vec: Vec<u8> = vec![0; 26];
        for c in s.chars() {
            vec[(c as u8 - b'a') as usize] += 1;
        }
        map.entry(vec).or_default().push(s);
    }

    map.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn normalize(mut groups: Vec<Vec<String>>) -> Vec<Vec<String>> {
        for g in groups.iter_mut() {
            g.sort();
        }
        groups.sort();
        groups
    }

    #[test]
    fn groups_anagrams() {
        let strs = vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string(),
        ];
        let expected = vec![
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
            vec!["nat".to_string(), "tan".to_string()],
            vec!["bat".to_string()],
        ];
        assert_eq!(normalize(group_anagrams(strs)), normalize(expected));
    }

    #[test]
    fn handles_empty_string() {
        let strs = vec!["".to_string()];
        let expected = vec![vec!["".to_string()]];
        assert_eq!(normalize(group_anagrams(strs)), normalize(expected));
    }

    #[test]
    fn single_word() {
        let strs = vec!["a".to_string()];
        let expected = vec![vec!["a".to_string()]];
        assert_eq!(normalize(group_anagrams(strs)), normalize(expected));
    }

    #[test]
    fn no_anagrams() {
        let strs = vec!["abc".to_string(), "def".to_string()];
        let expected = vec![vec!["abc".to_string()], vec!["def".to_string()]];
        assert_eq!(normalize(group_anagrams(strs)), normalize(expected));
    }
}
