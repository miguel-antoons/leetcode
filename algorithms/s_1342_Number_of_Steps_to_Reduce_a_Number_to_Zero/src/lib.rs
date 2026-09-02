// LeetCode problem: 1342. Number of Steps to Reduce a Number to Zero
use std::i32;

pub struct Solution;

impl Solution {
    pub fn number_of_steps(mut num: i32) -> i32 {
        let mut i = 0;
        while num > 0 {
            if num & 1 == 0 {
                num >>= 1;
            } else {
                num -= 1;
            }
            i += 1;
        }
        i
    }
}
