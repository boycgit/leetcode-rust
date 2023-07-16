use crate::从中序与后序遍历序列构造二叉树_106;
use crate::tree;
use std::cell::RefCell;
use std::rc::Rc;
use crate::utils::tree::{to_tree, TreeNode};

fn test_target(inorder: Vec<i32>, postorder: Vec<i32>, output: Option<Rc<RefCell<TreeNode>>> ) {
    assert_eq!(
        从中序与后序遍历序列构造二叉树_106::Solution::build_tree(inorder, postorder),
        output
    )
}

#[test]
fn 从中序与后序遍历序列构造二叉树_106_1() {
    test_target(vec![9,3,15,20,7], vec![9,15,7,20,3], tree![3,9,20,null,null,15,7])
}

#[test]
fn 从中序与后序遍历序列构造二叉树_106_2() {
    test_target(vec![-1], vec![-1], tree![-1])
}