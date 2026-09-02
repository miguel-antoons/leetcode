// LeetCode problem: 238. Product of Array Except Self

pub struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut prefix = Vec::new();
        let sz = nums.len();
        prefix.push(1);
        for i in 0..sz - 1 {
            prefix.push(prefix[prefix.len() - 1] * nums[i]);
        }
        
        let mut suffix = 1;
        for i in (0..sz).rev() {
            prefix[i] *= suffix;
            suffix *= nums[i];  // calculatenext suffix
        }

        prefix
    }
}
