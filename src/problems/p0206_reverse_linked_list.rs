use crate::common::ListNode;

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut curr = head;
    while let Some(mut node) = curr {
        curr = node.next.take();
        node.next = prev;
        prev = Some(node);
    }
    prev
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::list::{from_vec, to_vec};

    #[test]
    fn reverses_normal_list() {
        let input = from_vec(vec![1, 2, 3, 4, 5]);
        let result = reverse_list(input);
        assert_eq!(to_vec(result), vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn handles_empty_list() {
        let input = from_vec(vec![]);
        let result = reverse_list(input);
        assert_eq!(to_vec(result), vec![]);
    }

    #[test]
    fn handles_single_node() {
        let input = from_vec(vec![1]);
        let result = reverse_list(input);
        assert_eq!(to_vec(result), vec![1]);
    }
}
