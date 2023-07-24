use crate::最长递增子序列_300;

fn test_target(nums: Vec<i32>, output: i32 ) {
    assert_eq!(
        最长递增子序列_300::Solution::length_of_lis(nums),
        output
    )
}

#[test]
fn 最长递增子序列_300_1() {
    test_target(vec![10,9,2,5,3,7,101,18], 4)
}

#[test]
fn 最长递增子序列_300_2() {
    test_target(vec![0,1,0,3,2,3], 4)
}

#[test]
fn 最长递增子序列_300_3() {
    test_target(vec![4,10,4,3,8,9], 3)
}
#[test]
fn 最长递增子序列_300_4() {
    test_target(vec![7,7,7,7,7,7,7], 1)
}
#[test]
fn 最长递增子序列_300_5() {
    test_target(vec![1,3,6,7,9,4,10,5,6], 6)
}