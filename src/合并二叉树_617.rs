#[allow(dead_code)]
pub struct Solution {}
use crate::utils::tree::TreeNode;

/*
 * @lc app=leetcode.cn id=617 lang=rust
 *
 * [617] 合并二叉树
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
    pub fn merge_trees(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        match (root1.as_ref(), root2.as_ref()) {
            (None, None) => None,
            (Some(root1), None) => Some(root1.clone()),
            (None, Some(root2)) => Some(root2.clone()),
            (Some(root1), Some(root2)) => {
                let val = root1.borrow().val + root2.borrow().val;
                let base_node = Rc::new(RefCell::new(TreeNode::new(val)));


                base_node.borrow_mut().left = Self::merge_trees(root1.borrow().left.clone(), root2.borrow().left.clone());

                base_node.borrow_mut().right = Self::merge_trees(root1.borrow().right.clone(), root2.borrow().right.clone());

                Some(base_node)
            }
        }
    }
}
// @lc code=end

