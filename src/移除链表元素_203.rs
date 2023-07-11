#[allow(dead_code)]
pub struct Solution {}
use crate::utils::linked_list::{ListNode, Link};


/*
 * @lc app=leetcode.cn id=203 lang=rust
 *
 * [203] 移除链表元素
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
    pub fn remove_elements(head: Link, val: i32) -> Link {
       let mut dummy_head = Box::new(ListNode::new(0));
       dummy_head.next = head;

       let mut cur_node = dummy_head.as_mut();

       while let Some(node_next) = cur_node.next.take() {
            if node_next.val == val {
                cur_node.next = node_next.next;
            } else {
                cur_node.next = Some(node_next);
                cur_node = cur_node.next.as_mut().unwrap();
            }
       }

       dummy_head.next
    }
}
// @lc code=end

