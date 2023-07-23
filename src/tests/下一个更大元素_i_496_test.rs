use crate::下一个更大元素_i_496;

fn test_target(nums1: Vec<i32>, nums2: Vec<i32>, output: Vec<i32> ) {
    assert_eq!(
        下一个更大元素_i_496::Solution::next_greater_element(nums1, nums2),
        output
    )
}

#[test]
fn 下一个更大元素_i_496_1() {
    test_target(vec![4,1,2], vec![1,3,4,2], vec![-1,3,-1])
}

#[test]
fn 下一个更大元素_i_496_2() {
    test_target(vec![2, 4], vec![1,2,3,4], vec![3,-1])
}

#[test]
fn 下一个更大元素_i_496_3() {
    test_target(vec![4,1,2], vec![1,2,3,4], vec![-1,2,3])
}