use crate::判断子序列_392;

fn test_target(s: String, t: String, output: bool ) {
    assert_eq!(
        判断子序列_392::Solution::is_subsequence(s, t),
        output
    )
}

#[test]
fn 判断子序列_392_1() {
    test_target("abc".to_string(), "ahbgdc".to_string(), true)
}

#[test]
fn 判断子序列_392_2() {
    test_target("axc".to_string(), "ahbgdc".to_string(), false)
}

#[test]
fn 判断子序列_392_3() {
    test_target("".to_string(), "ahbgdc".to_string(), true)
}