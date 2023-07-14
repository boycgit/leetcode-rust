#[allow(dead_code)]
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=28 lang=rust
 *
 * [28] 找出字符串中第一个匹配项的下标
 */

// @lc code=start
impl Solution {
    pub fn get_next(needle: &Vec<char>) -> Vec<usize> {
        let mut next = vec![0; needle.len()];

        // 从 1 开始递归计算
        for i in 1..needle.len() {

            // 获取前一个公共前后缀最大数值，比如是 5
            let mut target_len = next[i-1];

            while target_len != 0  && needle[i] != needle[target_len] {
                // 如果字串前缀跟当前最后的最后不匹配，则退化找子串长度，如果一直找不到的话， target_len = 0
                target_len = next[target_len - 1]
            }

            // 如果找到的话，就是 target_len + 1
            next[i] = if needle[i] == needle[target_len] { target_len + 1 } else { target_len  };
        }

        next
    }

    pub fn str_str(haystack: String, needle: String) -> i32 {

        let n = (haystack).len();
        let m = (needle).len();

        // 如果模式串大于目标串，直接 -1
        if m > n {
            return -1;
        }

        let haystack = haystack.chars().collect::<Vec<char>>();
        let needle = needle.chars().collect::<Vec<char>>();

        // 获取 next 数组
        let next = Self::get_next(&needle);


        let mut i = 0;
        let mut j = 0;

        // 挨个查看主串的字符内容
        while i < n {
           while j < m && i < n &&  haystack[i] == needle[j] {
            i += 1;
            j += 1;
           }

           // 如果匹配完成
           if j == m {
            return (i - j) as i32;
           }

           // 如果 j > 0，说明 i 目前所在的字符是坏字符，需要让 j 移动到前缀处
           if j > 0 {
            j = next[j - 1];

           } else {
            // 到了此步 j == 0 说明 haystack[i] = p[0] 第一个字符串就匹配不上，直接让 i + 1 即可，进入下一个循环
            i += 1;
           }

        }

        // 到底了都还没判断出来，直接返回
        -1

    }
}
// @lc code=end

