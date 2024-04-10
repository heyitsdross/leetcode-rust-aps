pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

    for (i, i_val) in nums.iter().enumerate() {

        for (j, j_val) in nums[i+1..nums.len()].iter().enumerate() {
           if i_val + j_val == target {
               return vec![i as i32, (i+j+1) as i32];
           }
        }
    }
    panic!("No solution found for the two sum problem given vector {:?} and target {}", nums, target);
}

#[cfg(test)]
mod test;
