use std::collections::HashMap;

// time O(n), space O(n)
pub fn is_anagram(s: String, t: String) -> bool {
    let mut letter_counts: HashMap<char, i32> = HashMap::with_capacity(s.len());
    for s_c in s.chars() {
        letter_counts
            .entry(s_c)
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }

    for t_c in t.chars() {
        letter_counts
            .entry(t_c)
            .and_modify(|v| *v -= 1)
            .or_insert(-1);
    }

    letter_counts.values().all(|&v| v == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_anagram() {
        assert!(is_anagram("anagram".to_string(), "nagaram".to_string()));
    }

    #[test]
    fn detects_non_anagram() {
        assert!(!is_anagram("rat".to_string(), "car".to_string()));
    }

    #[test]
    fn handles_identical_strings() {
        assert!(is_anagram("abc".to_string(), "abc".to_string()));
    }

    #[test]
    fn handles_different_lengths() {
        assert!(!is_anagram("ab".to_string(), "abc".to_string()));
    }

    #[test]
    fn detects_non_anagram_with_repeated_letters() {
        assert!(!is_anagram("bbcc".to_string(), "ccbc".to_string()));
    }
}
