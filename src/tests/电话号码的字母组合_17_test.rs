use crate::电话号码的字母组合_17;

fn test_target(digits: String, output: Vec<String> ) {
    assert_eq!(
        电话号码的字母组合_17::Solution::letter_combinations(digits),
        output
    )
}

#[test]
fn 电话号码的字母组合_17_1() {
    test_target(String::from("23"), vec!["ad","ae","af","bd","be","bf","cd","ce","cf"].into_iter().map(String::from).collect())
}

#[test]
fn 电话号码的字母组合_17_2() {
    test_target("".to_string(),vec![])
}

#[test]
fn 电话号码的字母组合_17_3() {
    test_target("2".to_string(),vec!["a","b","c"].into_iter().map(String::from).collect())
}