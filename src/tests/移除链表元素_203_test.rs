use crate::{移除链表元素_203, linked};
use crate::utils::linked_list::{to_list, ListNode};

fn test_target(head: Option<Box<ListNode>>, val: i32, output: Option<Box<ListNode>> ) {
    assert_eq!(
        移除链表元素_203::Solution::remove_elements(head, val),
        output
    )
}

#[test]
fn 移除链表元素_203_1() {
    test_target(linked![1,2,6,3,4,5,6], 6, linked![1,2,3,4,5])
}

#[test]
fn 移除链表元素_203_2() {
    test_target(linked![], 1, linked![])
}

#[test]
fn 移除链表元素_203_3() {
    test_target(linked![7, 7, 7, 7], 7, linked![])
}