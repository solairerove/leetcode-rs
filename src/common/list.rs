#[derive(PartialEq, Eq, Debug, Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

pub fn from_vec(v: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    for &x in v.iter().rev() {
        let mut node = Box::new(ListNode::new(x));
        node.next = head;
        head = Some(node);
    }
    head
}

pub fn to_vec(mut head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut out = vec![];
    while let Some(node) = head {
        out.push(node.val);
        head = node.next;
    }
    out
}
