use crate::四数相加_ii_454;

fn test_target(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>, nums4: Vec<i32>, output: i32 ) {
    assert_eq!(
        四数相加_ii_454::Solution::four_sum_count(nums1, nums2, nums3, nums4),
        output
    )
}

#[test]
fn 四数相加_ii_454_1() {
    test_target(vec![1,2], vec![-2, -1], vec![-1,2],  vec![0, 2], 2)
}

#[test]
fn 四数相加_ii_454_2() {
    test_target(vec![0], vec![0], vec![0],  vec![0], 1)
}