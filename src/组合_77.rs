
#[allow(dead_code)]
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=77 lang=rust
 *
 * [77] 组合
 */

// @lc code=start
impl Solution {
    // 回溯算法
    pub fn backtracking(result: &mut Vec<Vec<i32>>, path: &mut Vec<i32>, n:i32, k: i32, start_idx: i32) {

        let path_len = path.len() as i32;
        // 返回的条件
        if path_len == k {
            result.push(path.to_vec());
            return;
        }

        // 每一步的横向选择集
        for j in start_idx..=(n-(k - path_len) + 1) {
            // 每一步的处理
            path.push(j);

            // 每一步的纵向选择集
            Self::backtracking(result, path, n, k, j + 1);
            // 消除次步的处理
            path.pop();
        }

    }

    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut path = vec![];

        Self::backtracking(&mut result, &mut path, n, k, 1);

        result
    }
}
// @lc code=end

