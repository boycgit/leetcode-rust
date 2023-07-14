use std::cell::RefCell;
use std::rc::Rc;
use crate::{二叉树的前序遍历_144, tree};
use crate::utils::tree::{to_tree, TreeNode};

fn test_target(root: Option<Rc<RefCell<TreeNode>>>, output: Vec<i32> ) {
    assert_eq!(
        二叉树的前序遍历_144::Solution::preorder_traversal(root),
        output
    )
}

#[test]
fn 二叉树的前序遍历_144_1() {
    test_target(tree![1,null,2,3], vec![1,2,3])
}

#[test]
fn 二叉树的前序遍历_144_2() {
    test_target(tree![], vec![])
}

#[test]
fn 二叉树的前序遍历_144_3() {
    test_target(tree![1], vec![1])
}

#[test]
fn 二叉树的前序遍历_144_4() {
    test_target(tree![1, null, 2], vec![1, 2])
}