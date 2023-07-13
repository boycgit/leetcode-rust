#[allow(dead_code)]
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [1] 两数之和
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        let mut map: HashMap<i32, i32> = HashMap::new();
        // 遍历数组
        for i in 0..nums.len() {
            let cur = nums[i];

            let res = target - cur;

            // 查询当前 cur 在 hashtable 中对应的 idx 下标，如果能查找到就说明存在两数之和
            if let Some(res_idx) = map.get(&cur) {
                return vec![*res_idx, i as i32];
            } else {
                // 否则就将当前 i 存入到 res 对应的 key 下
                *map.entry(res).or_insert(-1) = i as i32;
            }
        }

        vec![-1;2]
    }
}
// @lc code=end

