use crate::{删除链表的倒数第_n_个结点_19, linked};
use crate::utils::linked_list::{to_list, ListNode};


fn test_target(head: Option<Box<ListNode>>, n: i32, output: Option<Box<ListNode>> ) {
    assert_eq!(
        删除链表的倒数第_n_个结点_19::Solution::remove_nth_from_end(head, n),
        output
    )
}

#[test]
fn 删除链表的倒数第_n_个结点_19_1() {
    test_target(linked![1,2,3,4,5], 2, linked![1,2,3,5])
}

#[test]
fn 删除链表的倒数第_n_个结点_19_2() {
     test_target(linked![1], 1, linked![])
}

#[test]
fn 删除链表的倒数第_n_个结点_19_3() {
     test_target(linked![1, 2], 1, linked![1])
}