// LeetCode problem: 1331. Rank Transform of an Array
use std::vec::Vec;

pub struct Solution;

impl Solution {
    pub fn array_rank_transform(mut arr: Vec<i32>) -> Vec<i32> {
        if arr.is_empty() {
            return arr;
        }

        let mut indices = (0..arr.len()).collect::<Vec<_>>();
        indices.sort_by_key(|&i| &arr[i]);

        let mut current_rank = 1;
        let mut prev_val = arr[indices[0]];
        for i in indices {
            if arr[i] != prev_val {
                current_rank += 1;
            }
            prev_val = arr[i];
            arr[i] = current_rank;
        }
        
        arr
    }
}
