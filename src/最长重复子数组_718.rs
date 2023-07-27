#[allow(dead_code)]
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=718 lang=rust
 *
 * [718] 最长重复子数组
 */

// @lc code=start
impl Solution {
    pub fn find_length(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let len1 = nums1.len() + 1;
        let len2 = nums2.len() + 1;

        // dp 数组定义：以 nums1[i-1] 和 nums2[j-1] 为结尾的子数组长度
        let mut dp = vec![vec![0; len2];len1];

        let mut result = 0;

        for i in 1..len1 {
            for j in 1..len2 {

                // 如果数组最后一个元素相等
                if nums1[i-1] == nums2[j-1] {
                    dp[i][j] = dp[i-1][j-1] + 1;
                }
                result = result.max(dp[i][j]);
            }
            
        }

        result
    }
}
// @lc code=end

