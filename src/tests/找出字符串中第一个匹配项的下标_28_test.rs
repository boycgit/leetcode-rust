use crate::找出字符串中第一个匹配项的下标_28;

fn test_target(haystack: String, needle: String, output: i32 ) {
    assert_eq!(
        找出字符串中第一个匹配项的下标_28::Solution::str_str(haystack, needle),
        output
    )
}

#[test]
fn 找出字符串中第一个匹配项的下标_28_1() {
    test_target(String::from("sadbutsad"), String::from("sad"), 0)
}

#[test]
fn 找出字符串中第一个匹配项的下标_28_2() {
    test_target(String::from("leetcode"), String::from("leeto"), -1)
}

#[test]
fn 找出字符串中第一个匹配项的下标_28_3() {
    test_target(String::from("aaa"), String::from("aaaa"), -1)
}

#[test]
fn 找出字符串中第一个匹配项的下标_28_4() {
    test_target(String::from("mississippi"), String::from("issipi"), -1)
}