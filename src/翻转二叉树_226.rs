#[allow(dead_code)]
pub struct Solution {}
use crate::utils::tree::{TreeNode};

/*
 * @lc app=leetcode.cn id=226 lang=rust
 *
 * [226] 翻转二叉树
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
    // 后续遍历: 左右中
    pub fn traversal(node: Option<Rc<RefCell<TreeNode>>>) {
        
        if let Some(cur_node) = node.as_ref() {

            // 先获取左右节点
            let left = cur_node.borrow().left.clone();
            let right = cur_node.borrow().right.clone();

            // 左右递归
            Self::traversal(left.clone());
            Self::traversal(right.clone());

            // 反转当前节点的左右节点
            let tmp = left.clone();
            cur_node.borrow_mut().left = right.clone();
            cur_node.borrow_mut().right = tmp;

        }

    }
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        
        Self::traversal(root.clone());
        root
    }
}
// @lc code=end

