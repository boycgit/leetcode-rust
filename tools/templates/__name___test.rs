use crate::__name__;

fn test_target(__argsWithType__, output: __returnType__) {
    assert_eq!(
        __name__::Solution::__fnName__(__argsList__),
        output
    )
}

#[test]
fn __name___1() {
    test_target(vec![-1,0,3,5,9,12], 9, 4)
}

#[test]
fn __name___2() {
    test_target(vec![-1,0,3,5,9,12], 2, -1)
}