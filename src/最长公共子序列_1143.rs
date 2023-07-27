#[allow(dead_code)]
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=1143 lang=rust
 *
 * [1143] 最长公共子序列
 */

// @lc code=start
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let len1 = text1.len();
        let len2 = text2.len();

        let (s1, s2) = (text1.as_bytes(), text2.as_bytes());

        // dp[i][j] 表示 前 i-1 个字符和 j-1 个字符的最长公共子序列
        let mut dp = vec![vec![0; len2 + 1]; len1 + 1];

        for i in 1..=len1 {
            for j in 1..=len2 {
                if s1[i-1] == s2[j-1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = dp[i - 1][j].max(dp[i][j-1]);
                }
            }
        }

        dp[len1][len2]
    }
}
// @lc code=end

