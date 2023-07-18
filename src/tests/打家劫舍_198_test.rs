use crate::打家劫舍_198;

fn test_target(nums: Vec<i32>, output: i32 ) {
    assert_eq!(
        打家劫舍_198::Solution::rob(nums),
        output
    )
}

#[test]
fn 打家劫舍_198_1() {
    test_target(vec![1,2,3,1], 4)
}

#[test]
fn 打家劫舍_198_2() {
    test_target(vec![2,7,9,3,1], 12)
}