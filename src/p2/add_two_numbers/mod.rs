use std::fmt::Debug;

mod test;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Node {
    pub val: i32,
    pub next: Option<Box<Node>>,
}

impl Node {
    #[inline]
    fn new(val: i32) -> Self {
        Node { val, next: None }
    }
}

pub(crate) struct Solution {}

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<Node>>,
        l2: Option<Box<Node>>,
    ) -> Option<Box<Node>> {
        // Inputs never empty
        // No leading zeroes
        // Node.val always 0 <= val <= 9

        // Base case, neither value present and no carry over
        let mut v1 = l1.unwrap_or_else(|| Box::new(Node::new(0)));

        let v2 = l2.unwrap_or_else(|| Box::new(Node::new(0)));

        let v1c = v1.clone();
        let v1_next_c = v1.next.clone();
        let v1c2 = v1.clone();
        let v2c = v2.clone();

        let mut value = v1c.val + v2c.val;


        let mut assign = v1c.val.clone();
        // Carry value over to first number, or make first number if not there
        if value >= 10 {
            assign = match v1_next_c {
                Some(v1n) => v1n.val + 1,
                None => 1,
            };
            value = value - 10;
        }

        v1.next?.val = assign;
        
        if !v1c2.next.is_some() && v2.next.is_none() {
            Some(Box::new(Node::new(value)))
        } else {
            Some(Box::new(Node {
                val: value,
                next: Self::add_two_numbers(v1.next, v2.next),
            }))
        }
    }
}
