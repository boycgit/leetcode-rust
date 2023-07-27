use crate::最长公共子序列_1143;

fn test_target(text1: String, text2: String, output: i32 ) {
    assert_eq!(
        最长公共子序列_1143::Solution::longest_common_subsequence(text1, text2),
        output
    )
}

#[test]
fn 最长公共子序列_1143_1() {
    test_target("abcde".to_string(), "ace".to_string(), 3)
}

#[test]
fn 最长公共子序列_1143_2() {
    test_target("abc".to_string(), "abc".to_string(), 3)
}


#[test]
fn 最长公共子序列_1143_3() {
    test_target("abc".to_string(), "def".to_string(), 0)
}

