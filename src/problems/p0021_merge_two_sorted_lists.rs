use crate::common::ListNode;

pub fn merge_two_lists(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    let mut tail = &mut dummy;

    loop {
        match (list1.take(), list2.take()) {
            (Some(mut n1), Some(mut n2)) => {
                if n1.val <= n2.val {
                    list2 = Some(n2);
                    list1 = n1.next.take();
                    tail.next = Some(n1);
                } else {
                    list1 = Some(n1);
                    list2 = n2.next.take();
                    tail.next = Some(n2);
                }
                tail = tail.next.as_mut().unwrap();
                ()}
            (Some(n1), None) => {
                tail.next = Some(n1);
                break;
            }
            (None, Some(n2)) => {
                tail.next = Some(n2);
                break;
            }
            (None, None) => break,
        }
    }

    dummy.next
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
