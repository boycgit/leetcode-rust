use crate::测试_101;

fn test_target(nums: Vec<i32>, target: i32, output: i32) {
    assert_eq!(
        测试_101::Solution::search(nums, target),
        output
    )
}

#[test]
fn 测试_101_1() {
    test_target(vec![-1,0,3,5,9,12], 9, 4)
}

#[test]
fn 测试_101_2() {
    test_target(vec![-1,0,3,5,9,12], 2, -1)
}