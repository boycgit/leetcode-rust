#[allow(dead_code)]
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=977 lang=rust
 *
 * [977] 有序数组的平方
 */

// @lc code=start
impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut left = 0;
        let mut right  = nums.len() - 1;
        let mut result = vec![0; nums.len()];
        let mut k = nums.len() - 1;
        while left <= right {
            let left_2 = nums[left] * nums[left];
            let right_2 = nums[right] * nums[right];
            if left_2 < right_2 {
                result[k] = right_2;
                right -= 1;
            } else {
                result[k] = left_2;
                left += 1;
            }
            k -= 1;
        }
        result
    }
}
// @lc code=end

