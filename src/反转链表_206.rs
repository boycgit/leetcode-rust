#[allow(dead_code)]
pub struct Solution {}
use crate::utils::linked_list::{ListNode};


/*
 * @lc app=leetcode.cn id=206 lang=rust
 *
 * [206] 反转链表
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    // 遍历法
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }

        let mut cur = head;
        let mut pre = None;

        while let Some(mut cur_node) = cur.take() {

            cur = cur_node.next;
            cur_node.next = pre;
            pre = Some(cur_node)
        }

        pre
    }
}
// @lc code=end

