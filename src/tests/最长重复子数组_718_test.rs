use crate::最长重复子数组_718;

fn test_target(nums1: Vec<i32>, nums2: Vec<i32>, output: i32 ) {
    assert_eq!(
        最长重复子数组_718::Solution::find_length(nums1, nums2),
        output
    )
}

#[test]
fn 最长重复子数组_718_1() {
    test_target(vec![1,2,3,2,1], vec![3,2,1,4,7], 3)
}

#[test]
fn 最长重复子数组_718_2() {
    test_target(vec![0,0,0,0,0], vec![0,0,0,0,0], 5)
}