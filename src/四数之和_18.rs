#[allow(dead_code)]
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=18 lang=rust
 *
 * [18] 四数之和
 */

// @lc code=start
impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {

        let len = nums.len();
        let mut result = vec![];

        // 长度小于 4 的就直接范围空
        if len < 4 {
            return result;
        }


        let mut nums = nums;
        nums.sort();


        for i in 0..len  
        {
            let a = nums[i];

            // 最外层剪枝操作
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            for j in (i+1)..len {
                let b = nums[j];

                // 剪枝: ab
                if (a + b) > target && (a > 0 || target > 0) {
                    break;
                }

                // 次外层剪枝操作
                if j > i + 1 && nums[j] == nums[j - 1] {
                    continue;
                }

                let partial_sum = a + b;

                // 获取左右指针
                let mut left = j + 1;
                let mut right = len - 1;


                while left < right {
                    let c = nums[left];
                    let d = nums[right];
    
                    let sum = partial_sum + c + d;
                    // 如果等于目标数值
                    if sum == target {
                        result.push(vec![a, b, c, d]);
    
                        
                        while left < right && c == nums[left + 1] {
                            left += 1;
                        }
    
                        while left < right && d == nums[right - 1] {
                            right -= 1;
                        }
    
                        left += 1;
                        right -= 1;
                    } else if sum < target {
                        // 如果 sum 小于目标值，则让 left 坐标右移
                        left += 1;
                    } else {
                        right -= 1;
                    }
                }

        
            }
        }

        result
    }
}
// @lc code=end

