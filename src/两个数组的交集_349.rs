#[allow(dead_code)]
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=349 lang=rust
 *
 * [349] 两个数组的交集
 */

// @lc code=start
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut checks = vec![0;1001];

        let mut result = vec![];
        
        for i in 0..nums1.len() {
            checks[nums1[i as usize] as usize] = 1;
        }

        for i in 0..nums2.len() {
            if checks[nums2[i as usize] as usize] == 1 {
                // 获取之后需要将对应的位置置为 0，不然就重复取了
                checks[nums2[i as usize] as usize] = 0;
                result.push(nums2[i]);
            }
        }
        result

    }
}
// @lc code=end

