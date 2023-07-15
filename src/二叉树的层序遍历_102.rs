#[allow(dead_code)]
pub struct Solution {}
use crate::utils::tree::TreeNode;


/*
 * @lc app=leetcode.cn id=102 lang=rust
 *
 * [102] 二叉树的层序遍历
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
use std::vec;
use std::collections::VecDeque;
impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut queue = VecDeque::new();

        if root == None {
            return result;
        }

        queue.push_back(root);

        while !queue.is_empty() {
            
            let cur_len = queue.len();
            let mut cur_level = vec![];

            // 按当前已有的数量进行 for 子循环
            for _ in 0..cur_len {
                if let Some(cur_node) = queue.pop_front().unwrap() {
                    // 将当前值放入临时数组
                    cur_level.push(cur_node.borrow().val);
    
                    // 同时取出左右两边节点，放入队列
                    if let Some(left_node) = cur_node.borrow().left.clone() {
                        queue.push_back(Some(left_node))
                    }
                    if let Some(right_noc) = cur_node.borrow().right.clone() {
                        queue.push_back(Some(right_noc))
                    }
                    
                }
            }

            result.push(cur_level)
        }


        result
    }
}
// @lc code=end

