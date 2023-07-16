#[allow(dead_code)]
pub struct Solution {}
use crate::utils::tree::{TreeNode};


/*
 * @lc app=leetcode.cn id=222 lang=rust
 *
 * [222] 完全二叉树的节点个数
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

    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root.as_ref() {
            let (left, right) = (node.borrow().left.clone(), node.borrow().right.clone());

            // 遍历左右两边子树的深度，如果一致，就使用 2^h - 1 计算
            let mut depth_left = 0;
            let mut left_node = left.clone();
            while let Some(_)= left_node {
                left_node = left_node.unwrap().borrow().left.clone();
                depth_left += 1;
            }

            let mut depth_right = 0;
            let mut right_node = right.clone();
            while let Some(_)= right_node {
                right_node = right_node.unwrap().borrow().right.clone();
                depth_right += 1;
            }

            if depth_left == depth_right {
                return (2 << depth_left) - 1;
            } else {
                return Self::count_nodes(left) + Self::count_nodes(right) + 1;
            }
        } else {
            0
        }

    }
}
// @lc code=end

