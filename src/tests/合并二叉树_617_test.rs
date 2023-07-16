use crate::合并二叉树_617;
use crate::tree;
use std::cell::RefCell;
use std::rc::Rc;
use crate::utils::tree::{to_tree, TreeNode};

fn test_target(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>, output: Option<Rc<RefCell<TreeNode>>> ) {
    assert_eq!(
        合并二叉树_617::Solution::merge_trees(root1, root2),
        output
    )
}

#[test]
fn 合并二叉树_617_1() {
    test_target(tree![1,3,2,5], tree![2,1,3,null,4,null,7], tree![3,4,5,5,4,null,7])
}

#[test]
fn 合并二叉树_617_2() {
    test_target(tree![1], tree![1,2], tree![2,2])
}