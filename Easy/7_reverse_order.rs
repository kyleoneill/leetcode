/*
https://leetcode.com/problems/reverse-integer/
*/

impl Solution {
    pub fn reverse(mut x: i32) -> i32 {
        let mut out: i32 = 0;
        let mut negate = false;
        if x == 0 {
            return 0;
        }
        if x < 0 {
            negate = true;
            match x.checked_mul(-1) {
                Some(num) => x = num,
                None => return 0
            }
        }
        loop {
            let last_digit = x % 10;
            match out.checked_mul(10) {
                Some(num) => out = num,
                None => return 0
            }
            match out.checked_add(last_digit) {
                Some(num) => out = num,
                None => return 0
            }
            if x < 10 {
                break;
            }
            x /= 10;
        }
        if negate {
            match out.checked_mul(-1) {
                Some(num) => out = num,
                None => return 0
            }
        }
        out
    }
}