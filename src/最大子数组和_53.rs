#[allow(dead_code)]
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=53 lang=rust
 *
 * [53] 最大子数组和
 */

// @lc code=start
impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut dp = vec![0; len];

        let mut max_val = nums[0];

        dp[0] = nums[0];

        // dp[i] 数组定义：以 i 为结尾的最大子数组和
        for i in 1..nums.len() {
            // 当前值，跟历史最大值做一下对比，哪个大取哪个
            dp[i] = nums[i].max(dp[i - 1] + nums[i]);

            // 由于不一定是最后一个数组元素结尾的是最大和，所以需要每一步进行记录
            max_val = max_val.max(dp[i]);
        }

        max_val
    }
}
// @lc code=end

