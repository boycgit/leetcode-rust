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

        let mut maxLeft = vec![0; len];
        let mut maxRight = vec![0; len];

        maxLeft[0] = 0;
        maxRight[len - 1] = 0;

        // 动态规划更新这两个数组
        // 递推公式： maxLeft[i] = max(height[i - 1], maxLeft[i-1])
        // maxRight[i] = max(height[i+1], maxRight[i+1]);
        for i in 1..len {
            maxLeft[i] = max(height[i - 1], maxLeft[i-1]);

            let right_idx = len - 1 - i;
            maxRight[right_idx] = max(height[right_idx + 1], maxRight[right_idx + 1]);
        }

        // 然后再遍历整个数组计算出雨水总量
        // 某一点的计算公式：trap[i] = min(maxLeft[i], maxRight[i]) - height[i];
        for i in 0..len {
            result += max(0, min(maxLeft[i], maxRight[i]) - height[i]);
        }

        result
    }
}
// @lc code=end

