use crate::__name__;

fn test_target(nums: Vec<i32>, target: i32, output: i32) {
    assert_eq!(
        __name__::Solution::search(nums, target),
        output
    )
}

#[test]
fn __name___1() {
    test_target(vec![-1,0,3,5,9,12], 9, 4)
}

#[test]
fn __name___2() {
    test_target(vec![-1,0,3,5,9,12], 2, -1)
}