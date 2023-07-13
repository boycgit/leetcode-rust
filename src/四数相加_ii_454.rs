#[allow(dead_code)]
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=454 lang=rust
 *
 * [454] 四数相加 II
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn four_sum_count(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>, nums4: Vec<i32>) -> i32 {
        // 首先统计 A + B 的和的统计
        let mut map = HashMap::new();
        let mut result = 0;

        for i in 0..nums1.len() {
            for j in 0..nums2.len() {
                let sum_rev = -(nums1[i] + nums2[j]);
                *map.entry(sum_rev).or_insert(0) += 1;
            }
        }

        for i in 0..nums3.len() {
            for j in 0..nums4.len() {
                let sum = nums3[i] + nums4[j];
                let target_count = map.get(&sum).unwrap_or(&0);

                result += *target_count;
            }
        }

        result        
    }
}
// @lc code=end

