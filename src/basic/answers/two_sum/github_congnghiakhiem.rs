use std::collections::HashMap;
pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        
        let mut potential_matches = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            if let Some(&index) = potential_matches.get(&(target - num)) {
                return vec![index as i32, i as i32];
            }
            potential_matches.insert(num, i);
        }
        vec![]
    }
}

#[test]
fn test() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);

    let v = vec![-39, 7, -83, -30, 11, -55, -34, 11, -36, 85, -18, 67, 12, 10, -30, -87, 102, -35, 11, 14, -42, 109, 91, 95, 41, 21, -108, -23, -1, 14, -61, 9, 72, -52, -38, 80, -73, -108, 83, 92, 109];
    assert_eq!(Solution::two_sum(v, 35), vec![19, 25])

}
