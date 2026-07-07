use s_2236_Root_Equals_Sum_of_Children::Solution;
use s_2236_Root_Equals_Sum_of_Children::TreeNode;
use std::rc::Rc;
use std::cell::RefCell;

#[test]
fn test_check_tree_true() {
    let root = Rc::new(RefCell::new(TreeNode::new(10)));
    root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    assert_eq!(Solution::check_tree(Some(root)), true);
}

#[test]
fn test_check_tree_false() {
    let root = Rc::new(RefCell::new(TreeNode::new(10)));
    root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    assert_eq!(Solution::check_tree(Some(root)), false);
}

#[test]
fn test_check_tree_no_children() {
    let root = Rc::new(RefCell::new(TreeNode::new(5)));
    assert_eq!(Solution::check_tree(Some(root)), false);
}
