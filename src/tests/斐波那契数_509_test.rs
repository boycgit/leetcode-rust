use crate::斐波那契数_509;

fn test_target(n: i32, output: i32 ) {
    assert_eq!(
        斐波那契数_509::Solution::fib(n),
        output
    )
}

#[test]
fn 斐波那契数_509_1() {
    test_target(2, 1)
}

#[test]
fn 斐波那契数_509_2() {
    test_target(3, 2)
}

#[test]
fn 斐波那契数_509_3() {
    test_target(4, 3)
}