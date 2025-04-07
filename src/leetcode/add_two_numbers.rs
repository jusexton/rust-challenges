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

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut node_1 = l1;
    let mut node_2 = l2;
    let mut root_node: Option<Box<ListNode>> = None;
    let mut current_node: Option<&mut Box<ListNode>> = None;
    let mut carry = 0;
    loop {
        if node_1.is_none() && node_2.is_none() {
            break;
        }
        let node_1_value = node_1.as_ref().map_or(0, |node| node.val);
        let node_2_value = node_2.as_ref().map_or(0, |node| node.val);
        let sum = node_1_value + node_2_value + carry;
        let sum_digit = sum % 10;
        carry = sum / 10;
        let node = ListNode::new(sum_digit);
        current_node = Some(match current_node {
            Some(current_node_ref) => current_node_ref.next.insert(Box::new(node)),
            None => root_node.insert(Box::new(node)),
        });
        node_1 = node_1.and_then(|node| node.next);
        node_2 = node_2.and_then(|node| node.next);
    }
    if carry == 1 {
        #[allow(unused_assignments)]
        {
            current_node = Some(
                current_node
                    .unwrap()
                    .next
                    .insert(Box::new(ListNode::new(1))),
            );
        }
    }
    Some(match root_node {
        Some(node) => node,
        None => Box::new(ListNode::new(0)),
    })
}

#[cfg(test)]
mod test {
    use super::*;

    fn create_linked_list(iter: impl IntoIterator<Item = i32>) -> Option<Box<ListNode>> {
        let mut root_node = None;
        let mut current_node: Option<&mut Box<ListNode>> = None;
        for item in iter {
            let node = Box::new(ListNode::new(item));
            match current_node {
                Some(current_node_ref) => {
                    current_node = Some(current_node_ref.next.insert(node));
                }
                None => current_node = Some(root_node.insert(node)),
            }
        }
        root_node
    }

    #[test]
    fn test_1() {
        assert_eq!(
            add_two_numbers(create_linked_list([2, 4, 3]), create_linked_list([5, 6, 4])),
            create_linked_list([7, 0, 8])
        );
    }

    #[test]
    fn test_3() {
        assert_eq!(
            add_two_numbers(
                create_linked_list([9, 9, 9, 9, 9, 9, 9]),
                create_linked_list([9, 9, 9, 9])
            ),
            create_linked_list([8, 9, 9, 9, 0, 0, 0, 1])
        );
    }
}
