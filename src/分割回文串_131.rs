#[allow(dead_code)]
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=131 lang=rust
 *
 * [131] 分割回文串
 */

// @lc code=start
impl Solution {

    // 判断是否是回文字串
    pub fn is_sym(s: &Vec<char>, start: usize, end: usize) -> bool {
        let (mut start, mut end) = (start, end);
        
        while start < end {
            if s[start] != s[end] {
                return false;
            }
            
            start += 1;
            end -= 1;
        }

        true
    }

    pub fn backtracking(result: &mut Vec<Vec<String>>, path: &mut Vec<String>, origin_str: &Vec<char>, start_idx: usize) {


        // 如果起始位置大于s的大小，说明找到了一组分割方案
        if start_idx >= origin_str.len() {
            result.push(path.clone());
            return;
        }

        // 横向遍历集合
        for i in start_idx..origin_str.len() {

            if !Self::is_sym(origin_str, start_idx, i) {
                continue;
            }

            //如果是回文子串，则记录
            let s: String = origin_str[start_idx..i+1].into_iter().collect();
            path.push(s);

            Self::backtracking(result, path, origin_str, i + 1);

            path.pop();
        }

    }
    pub fn partition(s: String) -> Vec<Vec<String>> {

        let mut result = vec![];
        let mut path = vec![];

        let sub_str: Vec<char> = s.chars().collect();

        Self::backtracking(&mut result, &mut path, &sub_str, 0);

        result
    }
}
// @lc code=end

