#[allow(dead_code)]
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=383 lang=rust
 *
 * [383] 赎金信
 */

// @lc code=start
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut count = vec![0; 26];

        let base_idx = 'a' as u8;

        // 先统计 magazine 中的字符
        for c in magazine.chars() {
            count[(c as u8 - base_idx) as usize] += 1;
        }

        // 再统计 ransom_note 中的字符
        for c in ransom_note.chars() {
            count[(c as u8 - base_idx) as usize] -= 1;
        }
        count.into_iter().all(|val| val >= 0)

    }
}
// @lc code=end

