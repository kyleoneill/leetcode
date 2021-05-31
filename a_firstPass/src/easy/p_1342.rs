/*
num of steps to reduce a num to 0 - https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/
*/

pub fn solve() -> bool {
    let input = 14;
    let solution = number_of_steps(input);
    solution == 6
}

pub fn number_of_steps(num: i32) -> i32 {
    let mut steps = 0;
    let mut num = num;
    while num != 0 {
        if num % 2 == 0 {
            num /= 2;
        }
        else {
            num -= 1;
        }
        steps += 1;
    }
    steps
}