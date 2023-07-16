use crate::验证二叉搜索树_98;
use crate::tree;
use std::cell::RefCell;
use std::rc::Rc;
use crate::utils::tree::{to_tree, TreeNode};

fn test_target(root: Option<Rc<RefCell<TreeNode>>>, output: bool ) {
    assert_eq!(
        验证二叉搜索树_98::Solution::is_valid_bst(root),
        output
    )
}

#[test]
fn 验证二叉搜索树_98_1() {
    test_target(tree![2,1,3], true)
}

#[test]
fn 验证二叉搜索树_98_2() {
    test_target(tree![5,1,4,null,null,3,6], false)
}

#[test]
fn 验证二叉搜索树_98_3() {
    test_target(tree![5,4,6,null,null,3,7], false)
}

#[test]
fn 验证二叉搜索树_98_4() {
    test_target(tree![2, 2, 2], false)
}

#[test]
fn 验证二叉搜索树_98_5() {
    test_target(tree![-2147483648], true)
}