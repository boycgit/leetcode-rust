use crate::路径总和_112;
use crate::tree;
use std::cell::RefCell;
use std::rc::Rc;
use crate::utils::tree::{to_tree, TreeNode};

fn test_target(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32, output: bool ) {
    assert_eq!(
        路径总和_112::Solution::has_path_sum(root, target_sum),
        output
    )
}

#[test]
fn 路径总和_112_1() {
    test_target(tree![5,4,8,11,null,13,4,7,2,null,null,null,1], 22,  true)
}

#[test]
fn 路径总和_112_2() {
    test_target(tree![1,2,3], 5,false)
}

#[test]
fn 路径总和_112_3() {
    test_target(tree![], 0,false)
}