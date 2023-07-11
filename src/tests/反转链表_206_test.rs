use crate::{反转链表_206, linked};
use crate::utils::linked_list::{to_list, ListNode};

fn test_target(head: Option<Box<ListNode>>, output: Option<Box<ListNode>> ) {
    assert_eq!(
        反转链表_206::Solution::reverse_list(head),
        output
    )
}

#[test]
fn 反转链表_206_1() {
    test_target(linked![1,2], linked![2,1])
}

#[test]
fn 反转链表_206_2() {
    test_target(linked![], linked![])
}