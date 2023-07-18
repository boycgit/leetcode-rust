#[allow(dead_code)]
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=62 lang=rust
 *
 * [62] 不同路径
 */

// @lc code=start
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        // dp[i][j] 表示前往第 i, j 的路径数量
        let mut dp = vec![vec![0;n as usize]; m as usize];

        // 初始化横向、纵向两列
        dp[0][0] = 0;
        
        for i in 1..m {
            dp[i as usize][0] = 1;
        }

        for i in 1..n {
            dp[0][i as usize] = 1;
        }

        // 确定递推数组
        for i in 1..m {
            for j in 1..n {
                dp[i as usize][j as usize] = dp[i as usize - 1][j as usize] + dp[i as usize][j as usize - 1];
            }
        }

        dp[m as usize - 1][n as usize - 1]

    }
}
// @lc code=end

