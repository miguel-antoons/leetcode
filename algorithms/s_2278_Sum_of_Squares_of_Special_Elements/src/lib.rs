// LeetCode problem: 2278. Sum of Squares of Special Elements

pub struct Solution;

impl Solution {
    pub fn sum_of_squares(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        for (i, el) in nums.iter().enumerate() {
            if nums.len() % (i + 1) == 0 {
                sum += el * el;
            }
        }
        sum
    }
}
