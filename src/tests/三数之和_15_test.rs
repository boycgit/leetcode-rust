use crate::三数之和_15;

fn test_target(nums: Vec<i32>, output: Vec<Vec<i32>> ) {
    assert_eq!(
        三数之和_15::Solution::three_sum(nums),
        output
    )
}

#[test]
fn 三数之和_15_1() {
    test_target(vec![-1,0,1,2,-1,-4], vec![vec![-1,-1,2],vec![-1,0,1]])
}

#[test]
fn 三数之和_15_2() {
    test_target(vec![0,1,1], vec![])
}

#[test]
fn 三数之和_15_3() {
    test_target(vec![0, 0, 0], vec![vec![0,0,0]])
}
#[test]
fn 三数之和_15_4() {
    test_target(vec![0, 0, 0, 0], vec![vec![0,0,0]])
}