use crate::删除二叉搜索树中的节点_450;
use crate::tree;
use std::cell::RefCell;
use std::rc::Rc;
use crate::utils::tree::{to_tree, TreeNode};

fn test_target(root: Option<Rc<RefCell<TreeNode>>>, key: i32, output: Option<Rc<RefCell<TreeNode>>> ) {
    assert_eq!(
        删除二叉搜索树中的节点_450::Solution::delete_node(root, key),
        output
    )
}

#[test]
fn 删除二叉搜索树中的节点_450_1() {
    test_target(tree![5,3,6,2,4,null,7], 3, tree![5,4,6,2,null,null,7])
}

#[test]
fn 删除二叉搜索树中的节点_450_2() {
    test_target(tree![5,3,6,2,4,null,7], 0, tree![5,3,6,2,4,null,7])
}

#[test]
fn 删除二叉搜索树中的节点_450_3() {
    test_target(tree![], 0, tree![])
}