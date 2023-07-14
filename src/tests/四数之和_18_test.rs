use crate::四数之和_18;

fn test_target(nums: Vec<i32>, target: i32, output: Vec<Vec<i32>> ) {
    assert_eq!(
        四数之和_18::Solution::four_sum(nums, target),
        output
    )
}

#[test]
fn 四数之和_18_1() {
    test_target(vec![1,0,-1,0,-2,2], 0, vec![vec![-2,-1,1,2],vec![-2,0,0,2],vec![-1,0,0,1]])
}

#[test]
fn 四数之和_18_2() {
    test_target(vec![2,2,2,2,2], 8, vec![vec![2,2,2,2]])
}

#[test]
fn 四数之和_18_3() {
    test_target(vec![-2,-1,-1,1,1,2,2], 0, vec![vec![-2,-1,1,2], vec![-1,-1,1,1]])
}

#[test]
fn 四数之和_18_4() {
    test_target(vec![1000000000,1000000000,1000000000,1000000000], -294967296, vec![])
}