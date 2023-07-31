use crate::最大子数组和_53;

fn test_target(nums: Vec<i32>, output: i32 ) {
    assert_eq!(
        最大子数组和_53::Solution::max_sub_array(nums),
        output
    )
}

#[test]
fn 最大子数组和_53_1() {
    test_target(vec![-2,1,-3,4,-1,2,1,-5,4], 6)
}

#[test]
fn 最大子数组和_53_2() {
    test_target(vec![1], 1)
}