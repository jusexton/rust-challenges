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

type Node = Option<Rc<RefCell<TreeNode>>>;

pub fn sum_numbers(root: Node) -> i32 {
    let mut result = 0;

    fn dfs(node: Node, path: i32, result: &mut i32) {
        if let Some(n) = node {
            let n = n.borrow();
            if n.left.is_none() && n.right.is_none() {
                *result += path * 10 + n.val;
                return;
            }
            dfs(n.left.clone(), path * 10 + n.val, result);
            dfs(n.right.clone(), path * 10 + n.val, result);
        }
    }

    dfs(root, 0, &mut result);
    result
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::leetcode::sum_root_to_leaf_numbers::{TreeNode, sum_numbers};

    #[test]
    fn test_sum_numbers() {
        let root = Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        }));
        let actual = sum_numbers(Some(root));

        assert_eq!(25, actual);
    }
}
