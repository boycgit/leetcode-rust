#[allow(dead_code)]
pub struct Solution {}
use crate::utils::tree::TreeNode;


/*
 * @lc app=leetcode.cn id=145 lang=rust
 *
 * [145] 二叉树的后序遍历
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
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result = vec![];
        let mut stack = vec![];

        // 对于空树，需要先行判断
        if None == root {
            return result;
        }

        stack.push(root);
        while stack.len() > 0 {
            if let Some(cur) = stack.pop().unwrap() {
                let cur_node = cur.borrow();
                // 右、左、中，注意空节点不能入栈，不然就搞混了
                stack.push(Some(cur.clone()));
                stack.push(None);

                if let Some(right_node) = cur_node.right.clone() {
                    stack.push(Some(right_node));
                }

                if let Some(left_node) = cur_node.left.clone() {
                    stack.push(Some(left_node));
                }

            } else {
                let target_node = stack.pop().unwrap().unwrap();
                result.push(target_node.borrow().val);
            }
        }

        result
    }
}
// @lc code=end

