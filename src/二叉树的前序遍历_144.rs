#[allow(dead_code)]
pub struct Solution {}
use crate::utils::tree::TreeNode;

/*
 * @lc app=leetcode.cn id=144 lang=rust
 *
 * [144] 二叉树的前序遍历
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
    pub fn traversal(node: Option<Rc<RefCell<TreeNode>>>, result:&mut Vec<i32> ) {
        if let Some(node) = node {
            result.push(node.borrow().val); // 中
            Self::traversal(node.borrow().left.clone(), result); // 左
            Self::traversal(node.borrow().right.clone(), result); // 右
        }

    }
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        Self::traversal(root, &mut result);

        result
    }
}
// @lc code=end

