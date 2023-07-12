#[allow(dead_code)]
pub struct Solution {}
use crate::utils::linked_list::{ListNode};


/*
 * @lc app=leetcode.cn id=24 lang=rust
 *
 * [24] 两两交换链表中的节点
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
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = Box::new(ListNode::new(0));
        dummy_head.next = head;

        // 转变成指针引用
        let mut cur = dummy_head.as_mut();

        while let Some(mut next) = cur.next.take() {

            if let Some(mut next_2) = next.next.take() {

                next.next = next_2.next.take();
                next_2.next = Some(next);
                cur.next = Some(next_2);

                cur = cur.next.as_mut().unwrap().next.as_mut().unwrap()
            } else {
                cur.next = Some(next);
                cur = cur.next.as_mut().unwrap();
            }
        }

        dummy_head.next
    }
}
// @lc code=end

