use crate::二分查找_704;

fn test_target(nums: Vec<i32>, target: i32, output: i32) {
    assert_eq!(
        二分查找_704::Solution::search(nums, target),
        output
    )
}

#[test]
fn 二分查找_1() {
    test_target(vec![-1,0,3,5,9,12], 9, 4)
}

#[test]
fn 二分查找_2() {
    test_target(vec![-1,0,3,5,9,12], 2, -1)
}