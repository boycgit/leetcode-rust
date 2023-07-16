#[allow(dead_code)]
pub struct Solution {}
use crate::utils::tree::TreeNode;


/*
 * @lc app=leetcode.cn id=98 lang=rust
 *
 * [98] 验证二叉搜索树
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

    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        // 中序遍历之后，数值应该是递增排序
        let mut stack = vec![];

        // let mut pre_val = i32::MIN;
        // 记录前一个节点
        let mut pre_node: Option<Rc<RefCell<TreeNode>>> = None; 

        stack.push(root);

        while stack.len() > 0 {

            let node = stack.pop();

            // 中序遍历
            if let Some(node) = node.unwrap().as_ref() {

                if node.borrow().right.is_some() {
                    stack.push(node.borrow().right.clone());
                }

                stack.push(Some(node.clone()));
                stack.push(None);

                if node.borrow().left.is_some() {
                    stack.push(node.borrow().left.clone());
                }
            } else {
                let cur_node = stack.pop().unwrap();

                
                let cur_val = cur_node.clone().unwrap().borrow().val;

                if let Some(pre_node) = pre_node.as_ref() {
                    if pre_node.borrow().val >= cur_val {
                        return false;
                    }
                }
                pre_node = cur_node;
            }
            
        }
        
        true
    }
}
// @lc code=end

