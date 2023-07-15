use std::cell::RefCell;
use std::rc::Rc;
use crate::{二叉树的后序遍历_145, tree};
use crate::utils::tree::{to_tree, TreeNode};

fn test_target(root: Option<Rc<RefCell<TreeNode>>>, output: Vec<i32> ) {
    assert_eq!(
        二叉树的后序遍历_145::Solution::postorder_traversal(root),
        output
    )
}

#[test]
fn 二叉树的后序遍历_145_1() {
    test_target(tree![1,null,2,3], vec![3,2,1])
}

#[test]
fn 二叉树的后序遍历_145_2() {
    test_target(tree![], vec![])
}
#[test]
fn 二叉树的后序遍历_145_3() {
    test_target(tree![1], vec![1])
}