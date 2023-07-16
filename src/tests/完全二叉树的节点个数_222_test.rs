use crate::完全二叉树的节点个数_222;
use crate::tree;
use std::cell::RefCell;
use std::rc::Rc;
use crate::utils::tree::{to_tree, TreeNode};

fn test_target(root: Option<Rc<RefCell<TreeNode>>>, output: i32 ) {
    assert_eq!(
        完全二叉树的节点个数_222::Solution::count_nodes(root),
        output
    )
}

#[test]
fn 完全二叉树的节点个数_222_1() {
    test_target(tree![1,2,3,4,5,6], 6)
}

#[test]
fn 完全二叉树的节点个数_222_2() {
    test_target(tree![], 0)
}

#[test]
fn 完全二叉树的节点个数_222_3() {
    test_target(tree![1], 1)
}