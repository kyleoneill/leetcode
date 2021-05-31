/*
https://leetcode.com/problems/container-with-most-water/submissions/

*/

use std::cmp;

fn main() {
    let input = vec![1,8,6,2,5,4,8,3,7];
    let res = max_area_fast(input);
    println!("{}", res);
}

pub fn max_area_fast(height: Vec<i32>) -> i32 {
    let mut most_water: i32 = 0;
    let mut left = 0;
    let mut right = height.len() - 1;
    while left < right {
        let min_height = cmp::min(height[left], height[right]);
        let area = min_height * (right as i32 - left as i32);
        if area > most_water {
            most_water = area;
        }
        if height[left] < height[right] {
            let start = height[left];
            while left < right && height[left] <= start {
                left += 1;
            }
        } else {
            let start = height[right];
            while left < right && height[right] <= start {
                right -= 1;
            }
        }
    }
    most_water
}


/*
Pitfalls:
	-THIS IS SLOW
    -Running this without tallest_column causes it to take way too long
    -I tried to solve this by memoizing, adding column heights tested to a hashset
        -This did not solve the problem, so repeat heights was not the issue
    -Adding tallest_column fixed the problem, it works
		-Further columns in the iteration will have a smaller length,
		if the height is also smaller it is not possible for the area
		to be higher
*/
pub fn max_area_slow(height: Vec<i32>) -> i32 {
    let mut most_water: i32 = 0;
    let mut tallest_column = 0;
    for n in 0..height.len() {
        if height[n] < tallest_column {
            continue;
        }
        if height[n] > tallest_column {
            tallest_column = height[n];
        }
        for compare in 0..height.len() {
            if n == compare {
                continue;
            }
            let min_height = cmp::min(height[n], height[compare]);
            let area = min_height * (n as i32 - compare as i32).abs();
            if area > most_water {
                most_water = area;
            }
        }
    }
    most_water
}