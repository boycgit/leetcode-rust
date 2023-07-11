use crate::螺旋矩阵_ii_59;

fn test_target(n: i32, output: Vec<Vec<i32>> ) {
    assert_eq!(
        螺旋矩阵_ii_59::Solution::generate_matrix(n),
        output
    )
}

#[test]
fn 螺旋矩阵_ii_59_1() {
    test_target(3, vec![vec![1,2,3],vec![8,9,4],vec![7,6,5]])
}

#[test]
fn 螺旋矩阵_ii_59_2() {
    test_target(1, vec![vec![1]])
}