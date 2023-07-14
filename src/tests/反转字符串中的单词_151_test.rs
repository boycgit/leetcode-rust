use crate::反转字符串中的单词_151;

fn test_target(s: String, output: String ) {
    assert_eq!(
        反转字符串中的单词_151::Solution::reverse_words(s),
        output
    )
}

#[test]
fn 反转字符串中的单词_151_1() {
    test_target(String::from("the sky is blue"), String::from("blue is sky the"))
}

#[test]
fn 反转字符串中的单词_151_2() {
    test_target(String::from("  hello world  "), String::from("world hello"))
}

#[test]
fn 反转字符串中的单词_151_3() {
    test_target(String::from("a good   example"), String::from("example good a"))
}