
#[allow(dead_code)]
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=216 lang=rust
 *
 * [216] 组合总和 III
 */

// @lc code=start
impl Solution {
    pub fn backtracking(result: &mut Vec<Vec<i32>>, path: &mut Vec<i32>, target_sum: i32, k: i32, start_index: i32) {
        // 剪枝操作，如果某一路结果已经小于 0 了直接返回
        if target_sum < 0 {
            return;
        }

        // 当前路径长度
        let path_len = path.len() as i32;

        // 回溯中断条件
        if path_len == k {
            // 如果剩余数值是 0 ，说明找到目标路径了
            if target_sum == 0 {
                result.push(path.to_vec());
            }
            return;
        }

        // 横向可能情况
        // 如果 for 循环选择的起始位置之后的元素个数 已经不足 我们需要的元素个数了，那么就没有必要搜索了，目前还需要的元素至少个数是：k - path_len 个
        for i in start_index..= 9 - (k - path_len) + 1 {

            path.push(i);

            // 纵向查找，让 n - i 为探索的目标值
            Self::backtracking(result, path, target_sum - i, k, i + 1);

            path.pop();
        }

        
    }
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let mut path = vec![];

        Self::backtracking(&mut result, &mut path, n, k, 1);

        result
    }
}
// @lc code=end

