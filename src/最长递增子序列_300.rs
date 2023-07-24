#[allow(dead_code)]
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=300 lang=rust
 *
 * [300] 最长递增子序列
 */

// @lc code=start
impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        // 定义 dp 数组，dp[i] 表示以 i 结尾的，最长的子序列长度
        let len = nums.len();
        let mut dp = vec![1; len];

        // 还需要一个指标用来保存最大值
        let mut result = 1;

        // 递推公式，需要看 0..j 各种情况子序列的情况
        for i in 1..len {
            for j in 0..i {
                // 如果当前值比之前的要大，有机会新增
                if nums[i] > nums[j] {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }

            // 保存过程中的最大值
            result = result.max(dp[i]);
        }

        // println!("{:?}", dp);

        result
    }
}
// @lc code=end

