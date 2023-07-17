#[allow(dead_code)]
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=70 lang=rust
 *
 * [70] 爬楼梯
 */

// @lc code=start
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        // 确定 dp 数组的含义和初始化
        // dp[i] 表示到达 i 阶楼梯的方法数
        let mut dp = vec![0; (n + 2) as usize];
        dp[1] = 1;
        dp[2] = 2;

        if n >= 3 {
            let mut i = 3_usize;
    
            // 递推公式
            while i <= n as usize {
                dp[i] = dp[i-1] + dp[i-2];
                i += 1;
            }
        }

        // 最终返回的数值
        dp[n as usize]

    }
}
// @lc code=end

