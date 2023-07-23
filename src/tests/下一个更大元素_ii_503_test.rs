use crate::下一个更大元素_ii_503;

fn test_target(nums: Vec<i32>, output: Vec<i32> ) {
    assert_eq!(
        下一个更大元素_ii_503::Solution::next_greater_elements(nums),
        output
    )
}

#[test]
fn 下一个更大元素_ii_503_1() {
    test_target(vec![1,2,1], vec![2,-1,2])
}

#[test]
fn 下一个更大元素_ii_503_2() {
    test_target(vec![1,2,3,4,3], vec![2,3,4,-1,4])
}

#[test]
fn 下一个更大元素_ii_503_3() {
    test_target(vec![5,4,3,2,1], vec![-1,5,5,5,5])
}

#[test]
fn 下一个更大元素_ii_503_4() {
    test_target(vec![1,1,1,1,1], vec![-1,-1,-1,-1,-1])
}

#[test]
fn 下一个更大元素_ii_503_5() {
    test_target(vec![1,2,3,2,1], vec![2,3,-1,3,2])
}