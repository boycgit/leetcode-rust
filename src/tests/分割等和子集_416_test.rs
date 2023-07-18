use crate::分割等和子集_416;

fn test_target(nums: Vec<i32>, output: bool ) {
    assert_eq!(
        分割等和子集_416::Solution::can_partition(nums),
        output
    )
}

#[test]
fn 分割等和子集_416_1() {
    test_target(vec![1,5,11,5], true)
}

#[test]
fn 分割等和子集_416_2() {
    test_target(vec![1,2,3,5], false)
}