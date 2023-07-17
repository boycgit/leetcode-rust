use crate::爬楼梯_70;

fn test_target(n: i32, output: i32 ) {
    assert_eq!(
        爬楼梯_70::Solution::climb_stairs(n),
        output
    )
}

#[test]
fn 爬楼梯_70_1() {
    test_target(2, 2)
}

#[test]
fn 爬楼梯_70_2() {
    test_target(3, 3)
}