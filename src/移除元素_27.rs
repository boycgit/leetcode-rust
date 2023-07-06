#[allow(dead_code)]
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=27 lang=rust
 *
 * [27] 移除元素
 */

// @lc code=start
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
       let mut j = 0;

        for i in 0..nums.len() {
            if nums[i] == val {
                continue;
            } else {
                nums[j] = nums[i];
                j = j + 1;
            }
        }
        j as i32
    }
}
// @lc code=end

