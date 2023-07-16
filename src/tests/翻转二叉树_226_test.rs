use crate::{翻转二叉树_226, tree};
use std::cell::RefCell;
use std::rc::Rc;
use crate::utils::tree::{to_tree, TreeNode};


fn test_target(root: Option<Rc<RefCell<TreeNode>>>, output: Option<Rc<RefCell<TreeNode>>> ) {
    assert_eq!(
        翻转二叉树_226::Solution::invert_tree(root),
        output
    )
}

#[test]
fn 翻转二叉树_226_1() {
    test_target(tree![2,1,3], tree![2,3,1])
}

#[test]
fn 翻转二叉树_226_2() {
    test_target(tree![], tree![])
}