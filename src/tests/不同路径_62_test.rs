use crate::不同路径_62;

fn test_target(m: i32, n: i32, output: i32 ) {
    assert_eq!(
        不同路径_62::Solution::unique_paths(m, n),
        output
    )
}

#[test]
fn 不同路径_62_1() {
    test_target(3, 7, 28)
}

#[test]
fn 不同路径_62_2() {
    test_target(3, 2, 3)
}


#[test]
fn 不同路径_62_3() {
    test_target(7, 3, 28)
}

#[test]
fn 不同路径_62_4() {
    test_target(3, 3, 6)
}