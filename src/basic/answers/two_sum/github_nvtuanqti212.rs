// A recommended data structure for your answer
use std::collections::HashMap;
pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        for (index, value) in nums.iter().enumerate() {
            map.insert(value, index);
        }

        for (index, value) in nums.iter().enumerate() {
            let complement: i32 = target - value;
            if map.contains_key(&complement) && *map.get(&complement).unwrap() != index {
                return vec![index as i32, *map.get(&complement).unwrap() as i32];
            } else {
                continue;
            }
        }

        return vec![];
    }
}

#[test]
fn test() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1])
}
