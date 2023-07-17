#[allow(dead_code)]
pub struct Solution {}
use crate::utils::tree::TreeNode;


/*
 * @lc app=leetcode.cn id=669 lang=rust
 *
 * [669] 修剪二叉搜索树
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
    pub fn trim_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> Option<Rc<RefCell<TreeNode>>> {

        // 如果当前处理的点存在
        if let Some(root) = root.as_ref() {

            let val = root.borrow().val;
            let (left, right) = (root.borrow().left.clone(), root.borrow().right.clone());

						// 1. 
            // 如果当前值在区间左侧
            if val < low {
                // 查看其右子树（左子树的就不用检查了）
                if let Some(right_node) = right.as_ref() {
										// 1.a 
                    // 递归处理右子树
                    return Self::trim_bst(Some(right_node.clone()), low, high)
                    
                } else {
										// 1.b 
                    // 不存在右侧节点，则返回空
                    return None;
                }
            } else if val > high {
								// 2. 
                // 如果当前值在区间右侧，则查看其左子树数值（右子树的就不用检查了）
                if let Some(left_node) = left.as_ref() {
										// 2.a 递归处理左子树
                    return Self::trim_bst(Some(left_node.clone()), low, high);
                } else {
										// 2.b
                    return None;
                }

            } else {
								// 3. 
                // 如果在区间内就直接处理左右两边的节点
                root.borrow_mut().left = Self::trim_bst(left.clone(), low, high);

                root.borrow_mut().right = Self::trim_bst(right.clone(), low, high);

                return Some(root.clone());
            }
        }

        root
    }
}
// @lc code=end

