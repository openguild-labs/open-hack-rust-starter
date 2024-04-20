// A recommended data structure for your answer
use std::collections::HashMap;
pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Remove the below code and add your answer
        let mut map = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            if let Some(&j) = map.get(&(target - num)) {
                return vec![j as i32, i as i32];
            }
            map.insert(num, i);
        }
        vec![]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1])
}
