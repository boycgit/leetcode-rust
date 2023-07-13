use crate::两数之和_1;

fn test_target(nums: Vec<i32>, target: i32, output: Vec<i32> ) {
    assert_eq!(
        两数之和_1::Solution::two_sum(nums, target),
        output
    )
}

#[test]
fn 两数之和_1_1() {
    test_target(vec![2,7,11,15], 9, vec![0,1])
}

#[test]
fn 两数之和_1_2() {
    test_target(vec![3,2,4], 6, vec![1, 2])
}

#[test]
fn 两数之和_1_3() {
    test_target(vec![3,3], 6, vec![0, 1])
}