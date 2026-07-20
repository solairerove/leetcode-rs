use std::collections::HashMap;

pub fn is_anagram(s: String, t: String) -> bool {
    if s == t {
        return true;
    }

    if s.len() != t.len() {
        return false;
    }

    let mut letter_counts: HashMap<char, i32> = HashMap::with_capacity(s.len());
    let mut s_chars = s.chars();
    loop {
        match s_chars.next() {
            Some(s_c) => {
                if letter_counts.contains_key(&s_c) {
                    if let Some(v) = letter_counts.get_mut(&s_c) {
                        *v += 1;
                    }
                } else {
                    letter_counts.insert(s_c, 1);
                }
            }
            _ => {
                break;
            }
        }
    }

    let mut t_chars = t.chars();
    loop {
        match t_chars.next() {
            Some(t_c) => {
                if letter_counts.contains_key(&t_c) {
                    if let Some(v) = letter_counts.get_mut(&t_c) {
                        *v -= 1;
                    }
                } else {
                    return false;
                }
            }
            _ => {
                break;
            }
        }
    }

    for (_, v) in letter_counts {
        if v != 0 {
            return false;
        }
    }

    true
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
