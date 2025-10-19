#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn merge_nodes(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut curr = head.as_deref_mut();
    while let Some(curr_node) = curr.take() {
        let mut next_node = curr_node.next.take()?;
        let tail = next_node.next.take();
        curr_node.next = tail;

        if next_node.val == 0 {
            curr = curr_node.next.as_deref_mut();
        } else {
            curr_node.val += next_node.val;
            curr = Some(curr_node);
        }
    }
    head
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merges_values_between_zeroes() {
        let tail = Box::new(ListNode::new(0));
        let mut number = Box::new(ListNode::new(5));
        number.next = Some(tail);
        let mut head = Box::new(ListNode::new(0));
        head.next = Some(number);

        assert_eq!(Some(Box::new(ListNode::new(5))), merge_nodes(Some(head)))
    }
}
