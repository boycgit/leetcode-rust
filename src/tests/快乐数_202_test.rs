use crate::快乐数_202;

fn test_target(n: i32, output: bool ) {
    assert_eq!(
        快乐数_202::Solution::is_happy(n),
        output
    )
}

#[test]
fn 快乐数_202_1() {
    test_target(19, true)
}

#[test]
fn 快乐数_202_2() {
    test_target(2, false)
}