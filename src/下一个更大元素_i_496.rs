#[allow(dead_code)]
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=496 lang=rust
 *
 * [496] 下一个更大元素 I
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let len1 = nums1.len();
        let len2 = nums2.len();
        let mut result = vec![-1;len1];

        // 首先将 nums1 中的元素以 HashTable 方式存储
        // key 是具体的数值，value 是索引值
        let mut map = HashMap::new();
        for i in 0..len1 {
            map.insert(nums1[i], i);
        }


        // 进行单调栈处理
        let mut stack: Vec<&usize> = vec![];
        
        for i in 0..len2 {
            // 当前数值
            let cur = nums2[i];

            // 不断查看当前 stack 中的数值，跟当前数值进行比较
            while let Some(stack_idx) = stack.pop() {
                // 如果栈顶的元素比当前值小，说明找到了
                if nums1[*stack_idx] < cur {
                    // 在 result 中记录这个数值
                    result[*stack_idx] = cur;

                } else {
                    // 重新压栈回去，恢复场景，并中断查找
                    stack.push(stack_idx);
                    break;
                }
            }

            // 再查看这个值是否在 nums1 中
            // 如果是的话，需要将该值的索引存入 stack 中
            if let Some(idx) = map.get(&cur) {
                stack.push(idx);
            }


        }

        result
    }
}
// @lc code=end

