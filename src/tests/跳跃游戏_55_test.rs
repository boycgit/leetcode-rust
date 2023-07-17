use crate::跳跃游戏_55;

fn test_target(nums: Vec<i32>, output: bool ) {
    assert_eq!(
        跳跃游戏_55::Solution::can_jump(nums),
        output
    )
}

#[test]
fn 跳跃游戏_55_1() {
    test_target(vec![2,3,1,1,4], true)
}

#[test]
fn 跳跃游戏_55_2() {
    test_target(vec![3,2,1,0,4], false)
}
#[test]
fn 跳跃游戏_55_3() {
    test_target(vec![0,2,3], false)
}

#[test]
fn 跳跃游戏_55_4() {
    test_target(vec![1,2,3], true)
}