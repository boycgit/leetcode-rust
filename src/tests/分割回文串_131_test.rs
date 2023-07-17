use crate::分割回文串_131;

fn test_target(s: String, output: Vec<Vec<String>> ) {
    assert_eq!(
        分割回文串_131::Solution::partition(s),
        output
    )
}

#[test]
fn 分割回文串_131_1() {
    test_target("aab".to_string(), vec![vec!["a","a","b"].into_iter().map(String::from).collect(),vec!["aa","b"].into_iter().map(String::from).collect()])
}

#[test]
fn 分割回文串_131_2() {
    test_target("a".to_string(), vec![vec!["a".to_string()]])
}