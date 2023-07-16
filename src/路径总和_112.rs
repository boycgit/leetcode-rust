#[allow(dead_code)]
pub struct Solution {}
use crate::utils::tree::TreeNode;


/*
 * @lc app=leetcode.cn id=112 lang=rust
 *
 * [112] 路径总和
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

    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {

        if let Some(root) = root.as_ref() {
            // 剩余的和
            let target_sum = target_sum - root.borrow().val;

            // 获取左右节点
            let (left, right) = (root.borrow().left.clone(), root.borrow().right.clone());

            match (left.as_ref(), right.as_ref()) {
                (None, None) => return target_sum == 0,
                _ => (left.is_some() && Self::has_path_sum(left, target_sum)) || (right.is_some() &&Self::has_path_sum(right, target_sum))
            }

        } else {
            return false;
        }
        
    }
}
// @lc code=end

