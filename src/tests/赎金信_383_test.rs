use crate::赎金信_383;

fn test_target(ransom_note: String, magazine: String, output: bool ) {
    assert_eq!(
        赎金信_383::Solution::can_construct(ransom_note, magazine),
        output
    )
}

#[test]
fn 赎金信_383_1() {
    test_target(String::from("a"), String::from("b"), false)
}

#[test]
fn 赎金信_383_2() {
    test_target(String::from("aa"), String::from("ab"), false)
}

#[test]
fn 赎金信_383_3() {
    test_target(String::from("aa"), String::from("aab"), true)
}