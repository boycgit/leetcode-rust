#[allow(dead_code)]
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=42 lang=rust
 *
 * [42] 接雨水
 */

// @lc code=start
use std::cmp::{max, min};
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut result = 0;
        let len = height.len();

        // 单调栈
        let mut stack = vec![];

        // 比较当前元素跟栈内的元素
        for i in 0..len {
            let cur = height[i];

            // 根据栈顶元素做判断
            while let Some(top_idx) = stack.pop() {

                let top_num = height[top_idx];

                // 如果相等的话，只需要将当前 i 压入，去掉之前的元素
                if top_num == cur {
                    // 啥都不做，毕竟栈顶已经弹出了
                    break;
                } else if top_num > cur {
                    // 如果栈顶大于当前元素，恢复栈顶
                    stack.push(top_idx);
                    break;
                } else {
                    // 弹出元素直到比当前元素小，栈顶形成凹槽
                    // 检查是否还有左凹槽
                    if let Some(top_left_idx) = stack.last() {
                        // 水滴高度
                        let h = max(min(cur, height[*top_left_idx as usize]) - height[top_idx], 0);

                        // 水滴宽度
                        let w = i - *top_left_idx - 1;

                        // 更新结果
                        result += h * w as i32;
                    }
                }

            }

            // 判断完后无论如何都要将当前元素压入栈的
            stack.push(i);
        }

        result
    }
}
// @lc code=end

