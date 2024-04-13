use super::*;

#[test]
fn test_1() {
    let x = Some(Box::new(Node {
        val: 2,
        next: Some(Box::new(Node {
            val: 4,
            next: Some(Box::new(Node::new(3))),
        })),
    }));

    let y = Some(Box::new(Node {
        val: 5,
        next: Some(Box::new(Node {
            val: 6,
            next: Some(Box::new(Node::new(4))),
        })),
    }));

    let expected = Some(Box::new(Node {
        val: 7,
        next: Some(Box::new(Node {
            val: 0,
            next: Some(Box::new(Node::new(8))),
        })),
    }));

    assert_eq!(expected, Solution::add_two_numbers(x, y));
}

#[test]
fn test_zero_plus_zero_is_zero() {
    let x = Some(Box::new(Node::new(0)));
    let y = Some(Box::new(Node::new(0)));

    assert_eq!(
        Some(Box::new(Node::new(0))),
        Solution::add_two_numbers(x, y)
    );
}


#[test]
fn test_adding_nums_of_different_digits_carries_correctly() {
    let x = Some(Box::new(Node::new(0)));
    let y = Some(Box::new(Node::new(0)));

    assert_eq!(
        Some(Box::new(Node::new(0))),
        Solution::add_two_numbers(x, y)
    );
}

