#[allow(dead_code)]
pub struct Solution {}
use crate::utils::tree::TreeNode;


/*
 * @lc app=leetcode.cn id=106 lang=rust
 *
 * [106] 从中序与后序遍历序列构造二叉树
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
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        
        let len = postorder.len();

        // 在数据大小有值的情况下
        if len > 0 {
            // 获取后序数组的最后一个元素
            let base_num = postorder[len - 1];

            // 构造根节点
            let base_root = Rc::new(RefCell::new(TreeNode::new(base_num)));

            // 只有大于 1 个节点的时候才需要继续切割
            if len > 1 {
                // 切割中序数组
                let in_idx = inorder.iter().position(|&v| v == base_num).unwrap();
    
                // 两边同时进行构造
                base_root.as_ref().borrow_mut().left = Self::build_tree(inorder[0..in_idx].to_vec(), postorder[0..in_idx].to_vec());
    
                base_root.as_ref().borrow_mut().right = Self::build_tree(inorder[(in_idx + 1)..len].to_vec(), postorder[in_idx..(len-1)].to_vec());
            }

            // 返回根节点
            Some(base_root)
        } else {
            None
        }
    }
}
// @lc code=end

