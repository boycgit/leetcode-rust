#[allow(dead_code)]
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=59 lang=rust
 *
 * [59] 螺旋矩阵 II
 */

// @lc code=start
impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut res = vec![vec![0; n as usize]; n as usize];

        let mut count = 1;

        let mut k = 0;

        while k < (n + 1)/2 {
            let end_index = n-1-k;

            // 上横向
            for j in k..end_index {
                res[k as usize][j as usize] = count;
                count += 1;
            }

            // 右侧纵向
            for i in k..end_index {
                res[i as usize][end_index as usize] = count;
                count += 1;
            }

            // 下横向
            for j in (k..end_index).rev() {
                res[end_index as usize][(j + 1) as usize] = count;
                count += 1;
            }

            // 左侧纵向
            for i in (k..end_index).rev() {
                res[(i + 1) as usize][k as usize] = count;
                count += 1;
            }

            // 每次循环相当于少了两个步长
            k = k + 1;
        }

        // 最后一个中间元素
        if n % 2 == 1 {
            let half = (n / 2) as usize;
            res[half][half] = count;
        }

        res
    }
}
// @lc code=end

