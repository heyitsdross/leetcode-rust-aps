mod test;

pub(crate) struct Solution {}

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut max_depth = 0;
        let mut depth = 0;
        for char in s.chars() {
            let delta = match char {
                '(' => 1,
                ')' => -1,
                _ => 0,
            };
            if delta != 0 {
                depth = depth + delta;
                max_depth = std::cmp::max(max_depth, depth);
            }
        }

        max_depth
    }
}
