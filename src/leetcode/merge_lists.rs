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

pub fn merge_two_lists(
    list_one: Option<Box<ListNode>>,
    list_two: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (list_one, list_two) {
        (Some(left), None) => Some(left),
        (None, Some(right)) => Some(right),
        (None, None) => None,
        (Some(left), Some(right)) => {
            if left.val <= right.val {
                Some(Box::new(ListNode {
                    next: merge_two_lists(left.next, Some(right)),
                    val: left.val,
                }))
            } else {
                Some(Box::new(ListNode {
                    next: merge_two_lists(Some(left), right.next),
                    val: right.val,
                }))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::merge_lists::{merge_two_lists, ListNode};

    #[test]
    fn should_merge_two_lists() {
        let list_one = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode::new(3)))
        }));
        let list_two = Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode::new(4))),
        }));

        let actual = merge_two_lists(list_one, list_two);

        let expected = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode::new(4)))
                }))
            }))
        }));
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_merge_when_right_list_is_empty() {
        let list_one = Some(Box::new(ListNode::new(1)));
        let list_two = None;

        let actual = merge_two_lists(list_one, list_two);

        let expected = Some(Box::new(ListNode::new(1)));
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_merge_when_left_list_is_empty() {
        let list_one = None;
        let list_two = Some(Box::new(ListNode::new(1)));

        let actual = merge_two_lists(list_one, list_two);

        let expected = Some(Box::new(ListNode::new(1)));
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_return_none_when_both_lists_are_empty() {
        let list_one = None;
        let list_two = None;

        let actual = merge_two_lists(list_one, list_two);

        assert_eq!(None, actual);
    }
}
