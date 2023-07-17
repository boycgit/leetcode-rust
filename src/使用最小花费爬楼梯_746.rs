#[allow(dead_code)]
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=746 lang=rust
 *
 * [746] 使用最小花费爬楼梯
 */

// @lc code=start
use std::cmp::min;
impl Solution {
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {

        let len = cost.len();
        let mut dp = vec![0; cost.len() + 2];
        // 初始化 dp, dp[i] 表示到达第 i 个阶梯耗费最小的花费
        dp[0] = 0;
        dp[1] = 0;

        // 确定递推公式
        if len >= 2 {
            let mut i = 2 as usize;
             while i <= len {
                dp[i] = min(dp[i - 1] + cost[i-1], dp[i-2] + cost[i-2]);

                // println!("{}: {}", i, dp[i]);

                i += 1;

             }

        }

        dp[len]
    }
}
// @lc code=end

