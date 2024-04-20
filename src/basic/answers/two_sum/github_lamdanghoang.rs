// A recommended data structure for your answer
use std::collections::HashMap;
pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();

        for i in 0..(nums.len() - 1) {
            for j in (i + 1)..(nums.len()) {
                if target - nums[i] == nums[j] {
                    res.push(i as i32);
                    res.push(j as i32);
                }
            }
        }
        return res;
    }
}

#[test]
fn test() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]); //[2.7.11.15] =>[7.2.-2.-6]
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
}
