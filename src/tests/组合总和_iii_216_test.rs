use crate::{组合总和_iii_216, utils::set::array_to_hashset};

fn test_target(k: i32, n: i32, output: Vec<Vec<i32>> ) {
    assert_eq!(
        array_to_hashset::<_, 3>(组合总和_iii_216::Solution::combination_sum3(k, n)),
        array_to_hashset(output)
    )
}

#[test]
fn 组合总和_iii_216_1() {
    test_target(3, 7, vec![vec![1,2,4]])
}

#[test]
fn 组合总和_iii_216_2() {
    test_target(3, 9, vec![vec![1,2,6], vec![1,3,5], vec![2,3,4]])
}

#[test]
fn 组合总和_iii_216_3() {
    test_target(4, 1, vec![])
}

#[test]
fn 组合总和_iii_216_4() {
    test_target(9, 45, vec![vec![1,2,3,4,5,6,7,8,9]])
}