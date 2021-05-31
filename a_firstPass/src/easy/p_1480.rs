/*
Running Sum of 1d Array - https://leetcode.com/problems/running-sum-of-1d-array/
Given an array nums. We define a running sum of an array as runningSum[i] = sum(nums[0]…nums[i]).
Return the running sum of nums

constraints:
-1,000,000 <= nums[i] <= 1,000,000
1 <= nums.length <= 1000
*/

pub fn solve() -> bool {
    let input = vec![1, 2, 3, 4];
    let solution = running_sum(input);
    solution == vec![1, 3, 6, 10]
}

pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut output: Vec<i32> = Vec::new();
    output.push(nums[0]);
    if nums.len() > 1 {
        let mut position = 1;
        loop {
            if position >= nums.len() {
                break;
            }
            let sum = nums[position] + output.iter().last().unwrap();
            output.push(sum);
            position += 1;
        }
    }
    output
}

/*
More memory efficient solution - modify the input array in place, do not need an output array

        for (int i = 1; i < nums.size(); i++) {
            // Result at index `i` is sum of result at `i-1` and element at `i`.
            nums[i] += nums[i - 1];
        }
        return nums;
*/