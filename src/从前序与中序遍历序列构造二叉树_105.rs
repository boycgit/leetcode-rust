#[allow(dead_code)]
pub struct Solution {}
use crate::utils::tree::TreeNode;


/*
 * @lc app=leetcode.cn id=105 lang=rust
 *
 * [105] 从前序与中序遍历序列构造二叉树
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
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let len = preorder.len();

        if len > 0 {
            let base_num = preorder[0];
            
            let base_node = Rc::new(RefCell::new(TreeNode::new(base_num)));

            // 切割点索引值
            let idx = inorder.iter().position(|&v| v == base_num).unwrap();

            base_node.as_ref().borrow_mut().left = Self::build_tree(preorder[1..(idx + 1)].to_vec(), inorder[0..idx].to_vec());

            base_node.as_ref().borrow_mut().right = Self::build_tree(preorder[(idx + 1)..].to_vec(), inorder[(idx+1)..].to_vec());


            Some(base_node)
        } else {
            None
        }
    
    }
}
// @lc code=end

