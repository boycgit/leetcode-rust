use crate::移除元素_27;

fn test_target(nums:&mut Vec<i32>, target: i32, output: i32) {
    assert_eq!(
        移除元素_27::Solution::remove_element(nums, target),
        output
    )
}

#[test]
fn 移除元素_1() {
    test_target(&mut vec![3,2,2,3], 3, 2)
}

#[test]
fn 移除元素_2() {
    test_target(&mut vec![0,1,2,2,3,0,4,2], 2, 5)
}