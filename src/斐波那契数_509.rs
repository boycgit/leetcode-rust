#[allow(dead_code)]
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=509 lang=rust
 *
 * [509] 斐波那契数
 */

// @lc code=start
impl Solution {
    pub fn fib(n: i32) -> i32 {

        if n == 0 { return 0; }
        
        // dp 数组定义以及初始化
        let mut dp = vec![0; n as usize + 1];
        dp[1] = 1;

        let mut i = 2 as usize;

        // 递推公式
        while i <= n as usize {
            dp[i] = dp[i - 1] + dp[i - 2];
            i += 1;
        }
        dp[n as usize]
    }
}
// @lc code=end

