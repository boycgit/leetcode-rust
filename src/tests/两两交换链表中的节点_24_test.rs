use crate::{两两交换链表中的节点_24, linked};
use crate::utils::linked_list::{to_list, ListNode};


fn test_target(head: Option<Box<ListNode>>, output: Option<Box<ListNode>> ) {
    assert_eq!(
        两两交换链表中的节点_24::Solution::swap_pairs(head),
        output
    )
}

#[test]
fn 两两交换链表中的节点_24_1() {
    test_target(linked![1,2,3,4], linked![2,1,4,3])
}

#[test]
fn 两两交换链表中的节点_24_2() {
    test_target(linked![], linked![])
}
#[test]
fn 两两交换链表中的节点_24_3() {
    test_target(linked![1], linked![1])
}
#[test]
fn 两两交换链表中的节点_24_4() {
    test_target(linked![1,2,3], linked![2,1,3])
}