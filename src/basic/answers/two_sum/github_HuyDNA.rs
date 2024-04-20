use std::collections::HashMap;
pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i, num) in nums.into_iter().enumerate() {
            if map.contains_key(&(target - num)) {
                return vec![*map.get(&(target - num)).unwrap(), i as i32];
            } else {
                map.insert(num, i as i32);
            }
        }
        return vec![];
    }
}

#[test]
fn test() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1])
}
