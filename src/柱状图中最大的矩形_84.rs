#[allow(dead_code)]
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=84 lang=rust
 *
 * [84] 柱状图中最大的矩形
 */

// @lc code=start
use std::cmp::max;
impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let mut result = 0;

        // 数组前后加 0 
        let mut heights = heights;
        heights.push(0);
        heights.insert(0, 0);

        // 单调栈
        let mut stack = vec![];
        let len = heights.len();

        stack.push(0);

        for i in 1..len {
            let cur = heights[i];

            while let Some(top_idx) = stack.pop() {
                let top_num = heights[top_idx];

                // 如果当前值跟栈顶值相等，则抛弃这个栈顶的数值
                if cur == top_num {
                    break;
                } else if cur > top_num { 
                    // 当栈顶元素小于当前值
                    stack.push(top_idx); // 重新压栈
                    break;
                } else {
                    // 当栈顶元素大于当前值，说明找到右边界了
                    // 检查左边界是否存在
                    if let Some(top_left_idx) = stack.last() {
                        let h = top_num;
                        let w = (i - top_left_idx - 1) as i32;
                        result = max(result, w * h)
                    }
                }
            }

            stack.push(i);
            
        }

        result
    }
}
// @lc code=end

