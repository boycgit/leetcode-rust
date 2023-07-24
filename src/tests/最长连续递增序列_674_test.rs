use crate::最长连续递增序列_674;

fn test_target(nums: Vec<i32>, output: i32 ) {
    assert_eq!(
        最长连续递增序列_674::Solution::find_length_of_lcis(nums),
        output
    )
}

#[test]
fn 最长连续递增序列_674_1() {
    test_target(vec![1,3,5,4,7], 3)
}

#[test]
fn 最长连续递增序列_674_2() {
    test_target(vec![2,2,2,2,2], 1)
}