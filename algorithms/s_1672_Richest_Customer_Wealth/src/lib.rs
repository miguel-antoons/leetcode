// LeetCode problem: 1672. Richest Customer Wealth
use std::vec::Vec;

pub struct Solution;

impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let mut richest = 0;
        let mut sum;
        for acc in accounts {
            sum = 0;
            for amount in acc {
                sum += amount;
            }
            if sum > richest {
                richest = sum;
            }
        }
        richest
    }
}
