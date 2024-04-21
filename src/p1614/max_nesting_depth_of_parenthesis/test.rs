use crate::p1614::max_nesting_depth_of_parenthesis::Solution;

#[test]
fn test_case_1() {
    let vps = "(1+(2*3)+((8)/4))+1";
   assert_eq!(3, Solution::max_depth(String::from(vps)))
}

#[test]
fn test_case_2() {
    let vps = "(1+(2*3)+((8)/4))+1";
   assert_eq!(3, Solution::max_depth(String::from(vps)))
}