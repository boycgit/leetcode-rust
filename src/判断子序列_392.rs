#[allow(dead_code)]
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=392 lang=rust
 *
 * [392] 判断子序列
 */

// @lc code=start
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {

        let (s, t) = (s.as_bytes(), t.as_bytes());

        // dp[i][j] 表示以 i-1 的 s 子串是否是 j-1 的子串的子序列
        let len_s = s.len();
        let len_t = t.len();


        let mut dp = vec![vec![false; len_t + 1];len_s + 1];

        // s 是空字符串的话， 都是 t 的子序列
        for j in 0..=len_t {
            dp[0][j] = true;
        }

        for i in 1..=len_s {
            for j in 1..=len_t  {
                
                if s[i - 1] == t[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1];
                } else {
                    dp[i][j] = dp[i][j - 1];
                }
            }
        }
        
        // println!("{}, {}, {:?}", len_s, len_t, dp);

        dp[len_s][len_t]

    }
}
// @lc code=end

