use std::cell::RefCell;
use std::rc::Rc;
use crate::{二叉树的层序遍历_102, tree};
use crate::utils::tree::{to_tree, TreeNode};


fn test_target(root: Option<Rc<RefCell<TreeNode>>>, output: Vec<Vec<i32>> ) {
    assert_eq!(
        二叉树的层序遍历_102::Solution::level_order(root),
        output
    )
}

#[test]
fn 二叉树的层序遍历_102_1() {
    test_target(tree![3,9,20,null, null,15,7], vec![vec![3],vec![9,20],vec![15,7]]);
}

#[test]
fn 二叉树的层序遍历_102_2() {
        test_target(tree![1], vec![vec![1]]);
}

#[test]
fn 二叉树的层序遍历_102_3() {
        test_target(tree![], vec![]);
}