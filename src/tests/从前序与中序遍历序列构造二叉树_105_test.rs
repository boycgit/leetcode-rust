use crate::从前序与中序遍历序列构造二叉树_105;
use crate::tree;
use std::cell::RefCell;
use std::rc::Rc;
use crate::utils::tree::{to_tree, TreeNode};

fn test_target(preorder: Vec<i32>, inorder: Vec<i32>, output: Option<Rc<RefCell<TreeNode>>> ) {
    assert_eq!(
        从前序与中序遍历序列构造二叉树_105::Solution::build_tree(preorder, inorder),
        output
    )
}

#[test]
fn 从前序与中序遍历序列构造二叉树_105_1() {
    test_target(vec![3,9,20,15,7], vec![9,3,15,20,7], tree![3,9,20,null,null,15,7])
}

#[test]
fn 从前序与中序遍历序列构造二叉树_105_2() {
    test_target(vec![-1], vec![-1], tree![-1])
}