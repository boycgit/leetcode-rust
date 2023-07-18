#[allow(dead_code)]
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=416 lang=rust
 *
 * [416] 分割等和子集
 */

// @lc code=start
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let mut sum = 0;
        let len = nums.len();
        // 先计算出总和
        for i in 0..len {
            sum += nums[i];
        }

        // 先排除奇数情况
        if sum % 2 == 1 {return false;}

        // 然后找到目标和
        let target_sum = sum / 2;

        // 01 背包问题，初始化递归数组
        let mut dp = vec![0; target_sum as usize + 1];

        // 首先遍历元素个数
        for i in 0..len {
            let mut j = target_sum;
            // 逆向遍历
            let cur_num = nums[i];
            while j >= cur_num {
                dp[j as usize] = dp[j as usize].max(dp[j as usize - cur_num as usize] + cur_num);
                j -= 1;
            }
        }

        dp[target_sum as usize] == target_sum

    }
}
// @lc code=end

