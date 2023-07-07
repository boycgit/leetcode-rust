
#[allow(dead_code)]
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=209 lang=rust
 *
 * [209] 长度最小的子数组
 */

// @lc code=start
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = 0;
        let mut result = i32::max_value();
        let mut cur_sum = 0;
        while right < nums.len() {

            cur_sum += nums[right];

            while cur_sum >= target {
                cur_sum -= nums[left];
                let sub_len = (right - left + 1) as i32;
                if sub_len < result {
                    result = sub_len;
                }
                left += 1;
            }
            right += 1;
        }

        if result == i32::max_value() { 0 } else { result }
    }
}
// @lc code=end

