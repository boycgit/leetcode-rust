use crate::二叉树的最近公共祖先_236;
use crate::tree;
use std::cell::RefCell;
use std::rc::Rc;
use crate::utils::tree::{to_tree, TreeNode};

fn test_target(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>, output: Option<Rc<RefCell<TreeNode>>> ) {
    assert_eq!(
        二叉树的最近公共祖先_236::Solution::lowest_common_ancestor(root, p, q),
        output
    )
}

#[test]
fn 二叉树的最近公共祖先_236_1() {
    test_target(tree![3,5,1,6,2,0,8,null,null,7,4], tree![5,6,2,null,null,7,4], tree![1,0,8], tree![3,5,1,6,2,0,8,null,null,7,4])
}

#[test]
fn 二叉树的最近公共祖先_236_2() {
    test_target(tree![3,5,1,6,2,0,8,null,null,7,4], tree![5,6,2,null,null,7,4], tree![4], tree![5,6,2,null,null,7,4])
}
#[test]
fn 二叉树的最近公共祖先_236_3() {
    test_target(tree![1, 2], tree![1,2], tree![2], tree![1,2])
}