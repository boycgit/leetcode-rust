use crate::有效的字母异位词_242;

fn test_target(s: String, t: String, output: bool ) {
    assert_eq!(
        有效的字母异位词_242::Solution::is_anagram(s, t),
        output
    )
}

#[test]
fn 有效的字母异位词_242_1() {
    test_target(String::from("anagram"), String::from("nagaram"), true)
}

#[test]
fn 有效的字母异位词_242_2() {
    test_target(String::from("rat"), String::from("car"), false)
}