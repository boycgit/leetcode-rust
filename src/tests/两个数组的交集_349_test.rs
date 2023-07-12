use crate::两个数组的交集_349;

fn test_target(nums1: Vec<i32>, nums2: Vec<i32>, output: Vec<i32> ) {
    assert_eq!(
        两个数组的交集_349::Solution::intersection(nums1, nums2),
        output
    )
}

#[test]
fn 两个数组的交集_349_1() {
    test_target(vec![1,2,2,1], vec![2,2], vec![2])
}

#[test]
fn 两个数组的交集_349_2() {
    test_target(vec![4,9,5], vec![9,4,9,8,4], vec![9, 4])
}