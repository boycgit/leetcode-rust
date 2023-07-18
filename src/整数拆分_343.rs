#[allow(dead_code)]
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=343 lang=rust
 *
 * [343] 整数拆分
 */

// @lc code=start
use std::cmp::max;
impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        // dp 数组含义：dp[i] 表示数字 i 的最大整数和
        let mut dp = vec![0;n as usize + 1];
        dp[2] = 1; // 初始化

        // 递推公式
        for i in 3..dp.len() {
            for j in 1..=(i / 2) {
                dp[i as usize] = max(dp[i], max((i - j) * j, dp[i - j] * j))
            }
        }

        dp[n as usize] as i32
    }
}
// @lc code=end

