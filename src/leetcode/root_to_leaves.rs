use std::{cell::RefCell, rc::Rc};

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub fn is_valid_sequence(root: Option<Rc<RefCell<TreeNode>>>, arr: Vec<i32>) -> bool {
    let mut stack = Vec::new();
    stack.push((root, 0));

    while let Some((Some(current), index)) = stack.pop() {
        let current = current.borrow();
        if current.val != arr[index] {
            // This path is invalid, backtrack (skip to next item in stack).
            continue;
        }

        if index == arr.len() - 1 {
            if current.left.is_none() && current.right.is_none() {
                return true;
            }

            // Sequence matched, but ended too early (not a leaf), so continue backtracking.
            continue;
        }

        if let Some(ref right_child) = current.right {
            stack.push((Some(Rc::clone(right_child)), index + 1));
        }
        if let Some(ref left_child) = current.left {
            stack.push((Some(Rc::clone(left_child)), index + 1));
        }
    }

    // If the stack is empty and we haven't returned true, no valid path was found.
    false
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::leetcode::root_to_leaves::{TreeNode, is_valid_sequence};

    #[test]
    fn valid_sequence() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            }))),
        })));

        assert!(is_valid_sequence(root, vec![1, 5, 4]))
    }

    #[test]
    fn invalid_sequence() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
        })));

        assert!(!is_valid_sequence(root, vec![1, 4]))
    }
}
