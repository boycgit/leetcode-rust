#[allow(dead_code)]
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=17 lang=rust
 *
 * [17] 电话号码的字母组合
 */

// @lc code=start
impl Solution {

    pub fn backtracking(result: &mut Vec<String>, path: &mut Vec<char>, digits: &String,letter_map: &Vec<&str>, cur_index: i32) {


        let digits_len = digits.len() as i32;

        // 目标长度
        let path_len = path.len() as i32;

        // 达到目标之后中断回溯
        if path_len == digits_len {
            // 将 char 数组转换成 String 格式，并放入到 result 数组中
            result.push(path.iter().cloned().collect());
            return;
        }

        // 获取当前数字，比如 "2"
        let cur_num = digits.chars().nth(cur_index as usize).unwrap().to_digit(10).unwrap();

        // 获取字母集合，比如 "abc"
        let cur_str_sets = letter_map[cur_num as usize].to_string();

        // 本层集合元素（即每一个数字对应的字母范围，"abc" 范围中选）
        for c in cur_str_sets.chars() {

            // 处理节点
            path.push(c);
            
            // 递归，继续选择下一个字符情况
            Self::backtracking(result, path, digits, letter_map, cur_index + 1);

            path.pop(); // 回溯，撤销处理结果

        }
    }

    pub fn letter_combinations(digits: String) -> Vec<String> {

        if digits.len() == 0 {
            return vec![];
        }

        let letter_map = vec![
            "", "", "abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"
        ];

        let mut result = vec![];
        let mut path = vec![];

        Self::backtracking(&mut result, & mut path, &digits, &letter_map, 0);

        result

    }
}
// @lc code=end

