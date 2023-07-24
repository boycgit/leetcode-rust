#[allow(dead_code)]
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=674 lang=rust
 *
 * [674] 最长连续递增序列
 */

// @lc code=start
impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut result = 1;

        // dp 数组定义：以 nums[i] 结尾的最长连续子数组长度
        let mut dp = vec![1; len];
    
        for i in 1..len {
            if nums[i] > nums[i - 1] {
                dp[i] = dp[i - 1] + 1;
            }

            result = result.max(dp[i]);
        }

        result
    }
}
// @lc code=end

