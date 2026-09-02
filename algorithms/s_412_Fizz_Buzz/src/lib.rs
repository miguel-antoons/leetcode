// LeetCode problem: 412. Fizz Buzz
use std::vec::Vec;

pub struct Solution;

impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut res = Vec::new();
        for i in 1..n+1 {
            if (i % 15) == 0 {
                res.push(String::from("FizzBuzz"));
            } else if (i % 5) == 0 {
                res.push(String::from("Buzz"));
            } else if (i % 3) == 0 {
                res.push(String::from("Fizz"));
            } else {
                res.push(i.to_string());
            }
        }
        res
    }
}
