/*
richest customer wealth - https://leetcode.com/problems/richest-customer-wealth/
*/

pub fn solve() -> bool {
    let input = vec![vec![1,2,3], vec![3,2,1]];
    let solution = maximum_wealth(input);
    solution == 6
}

pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    let mut highest_wealth = 0;
    for customer in accounts {
        let mut total = 0;
        for bank in customer {
            total += bank;
        }
        if total > highest_wealth {
            highest_wealth = total;
        }
    }
    highest_wealth
}