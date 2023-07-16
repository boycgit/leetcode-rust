#[allow(dead_code)]
pub struct Solution {}
use crate::utils::tree::{TreeNode};


/*
 * @lc app=leetcode.cn id=101 lang=rust
 *
 * [101] 对称二叉树
 */

// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {

    // 同时进行两棵树的对比
    pub fn traversal_cmp(tree1: Option<Rc<RefCell<TreeNode>>>, tree2: Option<Rc<RefCell<TreeNode>>>) -> bool{

        // 通过 match 分条件进行判断中节点
        match (tree1.as_ref(), tree2.as_ref()) {
            (Some(tree1), Some(tree2)) => tree1.borrow().val == tree2.borrow().val 
            && Self::traversal_cmp(tree1.borrow().left.clone(), tree2.borrow().right.clone()) 
            && Self::traversal_cmp(tree1.borrow().right.clone(), tree2.borrow().left.clone()),
            (None, None) => true,
            _ => false,
        }
    }

    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let binding = root.unwrap();
        let root = binding.as_ref();

        let (left, right) = (root.borrow().left.clone(), root.borrow().right.clone());

        // 反转左边的子树
        Self::traversal_cmp(left, right)
    }
}
// @lc code=end

