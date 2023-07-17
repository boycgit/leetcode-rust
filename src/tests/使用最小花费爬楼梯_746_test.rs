use crate::使用最小花费爬楼梯_746;

fn test_target(cost: Vec<i32>, output: i32 ) {
    assert_eq!(
        使用最小花费爬楼梯_746::Solution::min_cost_climbing_stairs(cost),
        output
    )
}

#[test]
fn 使用最小花费爬楼梯_746_1() {
    test_target(vec![10,15,20], 15)
}

#[test]
fn 使用最小花费爬楼梯_746_2() {
    test_target(vec![1,100,1,1,1,100,1,1,100,1], 6)
}