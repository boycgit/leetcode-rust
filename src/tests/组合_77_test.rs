use crate::{组合_77, utils::set::array_to_hashset};

fn test_target(n: i32, k: i32, output: Vec<Vec<i32>> ) {
    assert_eq!(
        array_to_hashset::<_, 2>(组合_77::Solution::combine(n, k)),
        array_to_hashset(output)
    )
}

#[test]
fn 组合_77_1() {
    test_target(4, 2, vec![
  vec![2,4],
  vec![3,4],
  vec![2,3],
  vec![1,2],
  vec![1,3],
  vec![1,4],
])
}

#[test]
fn 组合_77_2() {
        test_target(1, 1, vec![
  vec![1]
])
}