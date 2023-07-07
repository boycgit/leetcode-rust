use crate::有序数组的平方_977;

fn test_target(nums: Vec<i32>, output: Vec<i32> ) {
    assert_eq!(
        有序数组的平方_977::Solution::sorted_squares(nums),
        output
    )
}

#[test]
fn 有序数组的平方_977_1() {
    test_target(vec![-4,-1,0,3,10], vec![0,1,9,16,100] )
}

#[test]
fn 有序数组的平方_977_2() {
    test_target(vec![-7,-3,2,3,11], vec![4,9,9,49,121])
}