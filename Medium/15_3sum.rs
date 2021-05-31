/*
https://leetcode.com/problems/3sum/
*/

use std::collections::HashSet;

impl Solution {
    pub fn two_sum(nums: &[i32], target: i32) -> HashSet<Vec<i32>> {
        let mut res: HashSet<Vec<i32>> = HashSet::new();
        let mut inverse: HashSet<i32> = HashSet::new();
        let mut doubles = 0;
        for i in 0..nums.len() {
            if 2 * nums[i] == target {
                doubles += 1;
            }
            else {
                inverse.insert(target - nums[i]);
            }
        }
        for i in 0..nums.len() {
            if 2 * nums[i] == target {
                continue;
            }
            else if inverse.contains(&nums[i]) {
                let mut new_member = vec![nums[i], target - nums[i]];
                new_member.sort();
                res.insert(new_member);
            }
        }
        if doubles > 1 {
            res.insert(vec![target / 2, target / 2]);
        }
        res
    }

    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut output: HashSet<Vec<i32>> = HashSet::new();
        if nums.len() < 3 {
            return vec![];
        }
        for i in 0..nums.len() - 2 {
            let target = nums[i] * -1;
            let two_sum = Self::two_sum(&nums[i + 1..], target);
            for mut sum in two_sum {
                sum.push(nums[i]);
                sum.sort();
                output.insert(sum);
            }
        }
        output.drain().collect()
    }
}