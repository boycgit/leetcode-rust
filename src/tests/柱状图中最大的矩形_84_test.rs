use crate::柱状图中最大的矩形_84;

fn test_target(heights: Vec<i32>, output: i32 ) {
    assert_eq!(
        柱状图中最大的矩形_84::Solution::largest_rectangle_area(heights),
        output
    )
}

#[test]
fn 柱状图中最大的矩形_84_1() {
    test_target(vec![2,1,5,6,2,3], 10)
}

#[test]
fn 柱状图中最大的矩形_84_2() {
    test_target(vec![2,4], 4)
}