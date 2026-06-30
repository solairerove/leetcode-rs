use crate::common::ListNode;

pub fn merge_two_lists(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (None, None) => None,
        (Some(n), None) | (None, Some(n)) => Some(n),
        (Some(mut n1), Some(mut n2)) => {
            if n1.val <= n2.val {
                n1.next = merge_two_lists(n1.next.take(), Some(n2));
                Some(n1)
            } else {
                n2.next = merge_two_lists(Some(n1), n2.next.take());
                Some(n2)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::list::{from_vec, to_vec};

    #[test]
    fn merges_interleaved() {
        let l1 = from_vec(vec![1, 2, 4]);
        let l2 = from_vec(vec![1, 3, 4]);
        let result = merge_two_lists(l1, l2);
        assert_eq!(to_vec(result), vec![1, 1, 2, 3, 4, 4]);
    }

    #[test]
    fn handles_both_empty() {
        assert_eq!(
            to_vec(merge_two_lists(from_vec(vec![]), from_vec(vec![]))),
            vec![]
        );
    }

    #[test]
    fn handles_one_empty() {
        let l1 = from_vec(vec![]);
        let l2 = from_vec(vec![0]);
        assert_eq!(to_vec(merge_two_lists(l1, l2)), vec![0]);
    }
}
