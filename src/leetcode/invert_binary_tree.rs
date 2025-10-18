use std::cell::RefCell;
use std::mem;
use std::rc::Rc;

pub type Node = Rc<RefCell<TreeNode>>;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Node>,
    pub right: Option<Node>,
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

pub fn invert_tree(root: Option<Node>) -> Option<Node> {
    let mut stack: Vec<Option<Node>> = vec![root.clone()];
    while let Some(item) = stack.pop() {
        if let Some(n) = item {
            let TreeNode { left, right, .. } = &mut *n.borrow_mut();
            mem::swap(right, left);
            stack.push(right.clone());
            stack.push(left.clone());
        }
    }
    root
}
