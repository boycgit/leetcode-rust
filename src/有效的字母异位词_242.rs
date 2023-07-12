#[allow(dead_code)]
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=242 lang=rust
 *
 * [242] 有效的字母异位词
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut map = HashMap::new();

        // 首先累加
        for c in s.chars() {
            *map.entry(c)
            .or_insert(0) += 1;
        }

        // 然后挨个累减
        for c in t.chars() {
            *map.entry(c).or_insert(0) -= 1;
        }

        // 判断 values 中值是否都为 0
        map.values().all(|&val| val == 0)
        
    }
}
// @lc code=end

