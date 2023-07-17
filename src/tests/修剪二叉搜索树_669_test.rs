use crate::修剪二叉搜索树_669;
use crate::tree;
use std::cell::RefCell;
use std::rc::Rc;
use crate::utils::tree::{to_tree, TreeNode};

fn test_target(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32, output: Option<Rc<RefCell<TreeNode>>> ) {
    assert_eq!(
        修剪二叉搜索树_669::Solution::trim_bst(root, low, high),
        output
    )
}

#[test]
fn 修剪二叉搜索树_669_1() {
    test_target(tree![1,0,2], 1, 2, tree![1, null, 2])
}

#[test]
fn 修剪二叉搜索树_669_2() {
   test_target(tree![3,0,4,null,2,null,null,1], 1, 3, tree![3,2,null,1])
}

#[test]
fn 修剪二叉搜索树_669_3() {
   test_target(tree![1,null,2], 2, 4, tree![2])
}

#[test]
fn 修剪二叉搜索树_669_4() {
   test_target(tree![3,2,4,1], 1, 1, tree![1])
}