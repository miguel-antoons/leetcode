use std::rc::Rc;
use std::cell::RefCell;


// Definition for a binary tree node.
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
      right: None
    }
  }
}

pub struct Solution {}

impl Solution {
    pub fn check_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root_rc) = &root {
            let root_borrow = root_rc.borrow();

            let left_val = if let Some(left_rc) = &root_borrow.left {
                left_rc.borrow().val
            } else {
                0
            };

            let right_val = if let Some(right_rc) = &root_borrow.right {
                right_rc.borrow().val
            } else {
                0
            };

            left_val + right_val == root_borrow.val
        } else {
            false
        }
    }
}
