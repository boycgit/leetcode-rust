#[allow(dead_code)]
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=55 lang=rust
 *
 * [55] 跳跃游戏
 */

// @lc code=start
use std::cmp::max;
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let len = nums.len() as i32;
        
        // 获取第一步跳的最大范围
        let mut target_idx = nums[0];

        let mut cur_idx = 0;

        // 不断更新范围内的最大 target_idx
        while cur_idx <= target_idx && cur_idx < len - 1 {
            target_idx  = max(cur_idx as i32 + nums[cur_idx as usize], target_idx);
            cur_idx += 1;
        }

        target_idx >= len as i32 - 1
    }
}
// @lc code=end

