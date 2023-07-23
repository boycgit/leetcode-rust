use crate::接雨水_42;

fn test_target(height: Vec<i32>, output: i32 ) {
    assert_eq!(
        接雨水_42::Solution::trap(height),
        output
    )
}

#[test]
fn 接雨水_42_1() {
    test_target(vec![0,1,0,2,1,0,1,3,2,1,2,1], 6)
}

#[test]
fn 接雨水_42_2() {
    test_target(vec![4,2,0,3,2,5], 9)
}