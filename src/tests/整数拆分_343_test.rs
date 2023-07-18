use crate::整数拆分_343;

fn test_target(n: i32, output: i32 ) {
    assert_eq!(
        整数拆分_343::Solution::integer_break(n),
        output
    )
}

#[test]
fn 整数拆分_343_1() {
    test_target(2, 1)
}

#[test]
fn 整数拆分_343_2() {
    test_target(10, 36)
}