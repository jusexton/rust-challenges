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

struct BSTIterator {
    current: Option<Rc<RefCell<TreeNode>>>,
    values: Vec<Rc<RefCell<TreeNode>>>,
}

impl BSTIterator {
    fn new(current: Option<Rc<RefCell<TreeNode>>>) -> Self {
        Self {
            current,
            values: vec![],
        }
    }

    fn next(&mut self) -> i32 {
        while let Some(curr_node) = self.current.take() {
            self.values.push(Rc::clone(&curr_node));
            self.current = curr_node.borrow_mut().left.take();
        }

        let node = self.values.pop().unwrap();
        let node_ref = node.borrow();
        self.current = node_ref.right.clone();
        node_ref.val
    }

    fn has_next(&self) -> bool {
        self.current.is_some() || !self.values.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::leetcode::binary_search_tree_iterator::{BSTIterator, TreeNode};

    #[test]
    fn test_binary_search_tree_iterator() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 15,
                left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(20)))),
            }))),
        })));

        let mut iter = BSTIterator::new(tree);

        assert_eq!(3, iter.next());
        assert_eq!(7, iter.next());
        assert!(iter.has_next());
        assert_eq!(9, iter.next());
        assert!(iter.has_next());
        assert_eq!(15, iter.next());
        assert!(iter.has_next());
        assert_eq!(20, iter.next());
        assert!(!iter.has_next());
    }
}
