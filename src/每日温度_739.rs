#[allow(dead_code)]
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=739 lang=rust
 *
 * [739] 每日温度
 */

// @lc code=start
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {

        let len = temperatures.len();
        let mut stack = vec![];

        // 首先将第 0 个元素放入栈中
        stack.push(0 as i32);

        let mut result = vec![0;len];

        for i in 1..len {
            if let Some(&last_ele) = stack.last() {
                let cur_temp = temperatures[i];
                // 查看最后一个元素是否大于之前的
                if temperatures[last_ele as usize] < cur_temp {
                    // 不断 pop 出对应的 index 
                    while let Some(cur_idx) = stack.pop() {

                        if temperatures[cur_idx as usize] < cur_temp {
                            // 更新目标数值
                            result[cur_idx as usize] = (i - cur_idx as usize) as i32;
                        } else {
                            stack.push(cur_idx); // 不满足条件的，要重新装回到栈中
                            break;
                        }
                    }
                    // 一直 pop 到没有，或者到栈中元素比他大了，就将当前 idx 放入栈
                    stack.push(i as i32);
                } else {
                    stack.push(i as i32);
                }
            }

        }


        result
    }
}
// @lc code=end

