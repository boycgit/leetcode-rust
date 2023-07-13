#[allow(dead_code)]
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=202 lang=rust
 *
 * [202] 快乐数
 */

// @lc code=start
use std::collections::HashSet;
impl Solution {
    pub fn is_happy(n: i32) -> bool {

        // 计算每个数值之和
        fn digit_sum(mut num: u32) -> u32 {
            let mut sum = 0_u32;
            while num != 0 {
                // 每位数的平方和
                sum += (num % 10).pow(2); 
                
                // 降低数值
                num = num / 10;
            }
            sum
        }

        let mut temp = n as u32;
        let mut set: HashSet<u32> = HashSet::new();

        while temp != 1 {
            temp = digit_sum(temp);
            
            // 如果之前 hash 里包含了这个元素，则说明会存在死循环，即不是快乐数
            if set.contains(&temp) {
                return false;
            }

            set.insert(temp);
        }

        return true;
    }
}
// @lc code=end

