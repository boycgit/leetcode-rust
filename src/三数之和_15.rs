#[allow(dead_code)]
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=15 lang=rust
 *
 * [15] 三数之和
 */

// @lc code=start
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums_sorted = nums.clone();
        let mut result = vec![];
        nums_sorted.sort();

        let nums_len = nums_sorted.len();

        for i in  0..nums_len {

            // 排序之后如果第一个元素已经大于零，那么无论如何组合都不可能凑成三元组，直接返回结果就可以了
            if (nums_sorted[i] > 0) { 
                return result;
            }

            // 如果存在重复元素（说明之前都判断过了，不需要再校验）
            if i > 0 && nums_sorted[i] == nums_sorted[i - 1] {
                continue;
            }

            // 获取左右双指针位置
            let mut left = i + 1;
            let mut right = nums_len - 1;

            // 双指针没有相遇
            while left < right {
                // 统计三个数的和
               let sum = nums_sorted[i] + nums_sorted[left] + nums_sorted[right];

                // 那么就存在一个符合条件的组合了
                if  sum == 0 {
                    result.push(vec![nums_sorted[i], nums_sorted[left], nums_sorted[right]]);

                    // 如果 left 数值一直相等就跳过（往右）
                    while left < right && nums_sorted[left] == nums_sorted[left + 1] {
                        left += 1;
                    }

                    left += 1;

                    // 如果 right 数值一直相等也跳过（往左）
                    while left < right && nums_sorted[right] == nums_sorted[right - 1] {
                        right -= 1;
                    }
                    right -= 1;

                } else if sum > 0 {
                    right -= 1;
                } else {
                    left += 1;
                } 
            }
        }
        result
    }
}
// @lc code=end

