#[allow(dead_code)]
pub struct Solution {}
use crate::utils::tree::TreeNode;


/*
 * @lc app=leetcode.cn id=450 lang=rust
 *
 * [450] 删除二叉搜索树中的节点
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


    pub fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {

        // 搜索到指定节点，然后进行删除
        if let Some(root) = root.as_ref() {

            let (left, right) = (root.borrow().left.clone(), root.borrow().right.clone());
            
            // 如果当前节点要被删除
            if root.borrow().val == key {
                return match (left, right) {
                    // 如果当前左右子节点不存在，就直接删除当前节点
                    (None, None) => None,

                    // 如果只存在左右节点
                    (Some(left), None) => Some(left),
                    (None, Some(right)) => Some(right),

                    // 如果左右节点都存在，则会相对比较复杂
                    // 需要将左节点放在右节点最左侧位置
                    (Some(left), Some(right)) => {
                        let mut right_left = right.clone();
                        while let Some(right_left_tmp) = right_left.clone().as_ref().borrow().left.clone() {
                            right_left = right_left_tmp;
                        }

                        // 直到左节点的尽头
                        right_left.borrow_mut().left = Some(left);
                        
                        // 返回右节点
                        return Some(right);
                    }
                }

            } else if root.borrow().val > key {
                // 否则根据 BST 性质， key 小于当前值则往左边找
                root.clone().borrow_mut().left = Self::delete_node(left.clone(), key);
            } else {
                // key 大于当前值则往左边找
                root.clone().borrow_mut().right = Self::delete_node(right.clone(), key);
            }
            
        }

        root

    }
}
// @lc code=end

