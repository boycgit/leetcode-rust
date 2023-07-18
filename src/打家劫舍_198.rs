#[allow(dead_code)]
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=198 lang=rust
 *
 * [198] 打家劫舍
 */

// @lc code=start
use std::cmp::max;
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        // dp 数组定义：下标 i 以内的房屋，最多可以偷取的金额
        let mut dp = vec![0; len];
        dp[0] = nums[0];

        if len < 2 {return dp[0];}

        dp[1] = dp[0].max(nums[1]);

        for i in 2..len {
            dp[i] = max(dp[i - 1], dp[i - 2] + nums[i]);
        }


        dp[len -1]
        
    }
}
// @lc code=end

