use crate::每日温度_739;

fn test_target(temperatures: Vec<i32>, output: Vec<i32> ) {
    assert_eq!(
        每日温度_739::Solution::daily_temperatures(temperatures),
        output
    )
}

#[test]
fn 每日温度_739_1() {
    test_target(vec![73,74,75,71,69,72,76,73], vec![1,1,4,2,1,1,0,0])
}

#[test]
fn 每日温度_739_2() {
    test_target(vec![30,40,50,60], vec![1,1,1,0])
}

#[test]
fn 每日温度_739_3() {
    test_target(vec![30,60,90], vec![1,1,0])
}