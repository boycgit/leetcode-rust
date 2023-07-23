
#[allow(dead_code)]
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=503 lang=rust
 *
 * [503] 下一个更大元素 II
 */

// @lc code=start
impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len();
        let mut result = vec![-1; len];

        let mut stack = vec![];

        for i in 0..len * 2 - 1 {
            let cur = nums[i % len];
            // 不断进行元素 pop
            while let Some(top_idx) = stack.pop() {
                // 如果找到比当前元素大的
                if nums[top_idx] < cur {
                    result[top_idx] = cur;
                } else {
                    stack.push(top_idx); // 重新入栈
                    break;
                }
            }

            // 将当前索引压入栈
            stack.push(i % len);
        }

        result
    }
}
// @lc code=end

