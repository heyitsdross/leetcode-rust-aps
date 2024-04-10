use super::*;

#[test]
pub fn test_two_sum_example() {
    let numbers = vec![2,7,11,15];
    let target = 9;

    assert_eq!(two_sum(numbers, target), vec![0, 1]);
}

#[test]
pub fn test_two_sum_example_two() {
    let numbers = vec![3, 2, 4];
    let target = 6;

    assert_eq!(two_sum(numbers, target), vec![1, 2]);
}

#[test]
pub fn test_two_sum_example_three() {
    let numbers = vec![3,3];
    let target = 6;

    assert_eq!(two_sum(numbers, target), vec![0, 1]);
}