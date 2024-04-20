// A recommended data structure for your answer
use std::collections::HashMap;
pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Remove the below code and add your answer
        let mut hmap = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            if hmap.contains_key(&(target - num)) {
                return vec![*hmap.get(&(target - num)).unwrap(), i as i32];
            } else {
                hmap.insert(num, i as i32);
            }
        }
        return vec![];
    }
}

#[test]
fn test() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1])
}
