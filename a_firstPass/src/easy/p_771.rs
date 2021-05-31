/*
Jewels and stones - https://leetcode.com/problems/jewels-and-stones/
*/

use std::collections::HashSet;

pub fn solve() -> bool {
    let jewels = "aA".to_owned();
    let stones = "aAAbbbb".to_owned();
    let solution = num_jewels_in_stones(jewels, stones);
    solution == 3
}

pub fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    let mut jewel_map: HashSet<char> = HashSet::new();
    let mut count = 0;
    for c in jewels.chars() {
        jewel_map.insert(c);
    }
    for c in stones.chars() {
        if jewel_map.contains(&c) {
            count += 1;
        }
    }
    count
}