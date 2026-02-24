use std::cell::RefCell;
use std::rc::Rc;

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

pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, current_sum: i32) -> i32 {
        if let Some(n) = node {
            let n = n.borrow();
            let new_sum = (current_sum << 1) | n.val;

            if n.left.is_none() && n.right.is_none() {
                return new_sum;
            }

            dfs(n.left.clone(), new_sum) + dfs(n.right.clone(), new_sum)
        } else {
            0
        }
    }

    dfs(root, 0)
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::leetcode::sum_of_root_to_leaf_binary_numbers::{TreeNode, sum_root_to_leaf};

    #[test]
    fn test_sum_root_to_leaf() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            }))),
        })));

        assert_eq!(22, sum_root_to_leaf(tree))
    }

    #[test]
    fn test_empty_tree() {
        assert_eq!(0, sum_root_to_leaf(None))
    }
}
