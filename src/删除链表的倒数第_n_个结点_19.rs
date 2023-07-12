#[allow(dead_code)]
pub struct Solution {}
use crate::utils::linked_list::{ListNode};

/*
 * @lc app=leetcode.cn id=19 lang=rust
 *
 * [19] 删除链表的倒数第 N 个结点
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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy_head = Box::new(ListNode::new(0));
        dummy_head.next = head;

        
        // 快指针，这里要先写快指针，不然 &dummy_head 会报错（因为下方的慢指针采用 &mut 引用）
        let mut sec_pos = &dummy_head.clone();
        // 慢指针
        let mut first_pos = &mut dummy_head;
        

        // 先让第二个指针（快指针）移动 n 步
        for _ in 0..n {
            sec_pos = sec_pos.next.as_ref().unwrap();
        }

        // 然后一直往后移动节点，直到 sec_pos 的 next 是 None 为止
        while let Some(_) = sec_pos.next.as_ref() {
            // 同时移动 first_pos 和 sec_pos
            sec_pos = sec_pos.next.as_ref().unwrap();
            first_pos = first_pos.next.as_mut().unwrap();
        }

        first_pos.next = first_pos.next.as_mut().unwrap().next.take();

        dummy_head.next
    }
}
// @lc code=end

