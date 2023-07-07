use crate::长度最小的子数组_209;

fn test_target(target: i32, nums: Vec<i32>, output: i32 ) {
    assert_eq!(
        长度最小的子数组_209::Solution::min_sub_array_len(target, nums),
        output
    )
}

#[test]
fn 长度最小的子数组_209_1() {
    test_target(7, vec![2,3,1,2,4,3], 2)
}

#[test]
fn 长度最小的子数组_209_2() {
    test_target(4, vec![1,4,4], 1)
}

#[test]
fn 长度最小的子数组_209_3() {
    test_target(11, vec![1,2,3,4,5], 3)
}