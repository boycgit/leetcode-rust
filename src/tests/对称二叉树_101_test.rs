use crate::对称二叉树_101;
use crate::tree;
use std::cell::RefCell;
use std::rc::Rc;
use crate::utils::tree::{to_tree, TreeNode};

fn test_target(root: Option<Rc<RefCell<TreeNode>>>, output: bool ) {
    assert_eq!(
        对称二叉树_101::Solution::is_symmetric(root),
        output
    )
}

#[test]
fn 对称二叉树_101_1() {
    test_target(tree![1,2,2,3,4,4,3], true)
}

#[test]
fn 对称二叉树_101_2() {
    test_target(tree![1,2,2,null,3,null,3], false)
}