// LeetCode problem: 3886. Sum of Sortable Integers

pub struct Solution;
pub struct SolutionOld;

struct CommonDivisors {
    initial_number: u32,
    current: u32,
    next: u32,
}

impl CommonDivisors {
    fn new(initial_number: u32) -> Self {
        Self {
            initial_number: initial_number,
            current: initial_number.isqrt(),
            next: initial_number,
        }
    }
}


impl Iterator for CommonDivisors {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next != 0 {
            let candidate = self.next;
            self.next = 0;
            return Some(candidate);
        }
        
        while self.current > 1 {
            let candidate = self.current;
            self.current -= 1;
            if self.initial_number % candidate == 0 {
                let pair = self.initial_number / candidate;
                if pair != candidate {
                    self.next = pair;
                }
                return Some(candidate);
            }
        }
        None
    }
}


impl Solution {
    pub fn sortable_integers(nums: Vec<i32>) -> i32 {
        let n = nums.len() as u32;
        if n == 1 {
            return 1;
        }
        
        let mut decreases = vec![0u32; nums.len()];
        let mut sorted = true;
        for i in 1..nums.len() {
            let decrease = nums[i - 1] > nums[i];
            sorted &= !decrease;
            decreases[i] = decreases[i - 1] + decrease as u32;
        }

        if sorted {
            return CommonDivisors::new(n).sum::<u32>() as i32 + 1;
        }

        let mut next_decrease = vec![n - 1; nums.len()];
        for i in (0..nums.len() - 1).rev() {
            next_decrease[i] = if nums[i] > nums[i + 1] { i as u32 } else { next_decrease[i + 1] };
        }
        
        let mut res = 0;
        let max_no_descents = decreases[nums.len() - 1];
        for div in CommonDivisors::new(n) {
            if div > n / max_no_descents {
                continue;
            }
            
            let mut prev_max = i32::MIN;
            let mut is_sortable = true;
            for leftmost in (0..nums.len()).step_by(div as usize) {
                let rightmost = leftmost + div as usize - 1;
                let internal_decreases = decreases[rightmost] - decreases[leftmost];
                let wrap_decrease = (nums[rightmost] > nums[leftmost]) as u32;
                if internal_decreases + wrap_decrease > 1 {
                    is_sortable = false;
                    break;
                }

                let (min, max) = if internal_decreases == 1 {
                    let split = next_decrease[leftmost] as usize;
                    (nums[leftmost].min(nums[split + 1]), nums[rightmost].max(nums[split]))
                } else {
                    (nums[leftmost], nums[rightmost])
                };
                // let max = nums[rightmost].max(nums[split]);
                if min < prev_max {
                    is_sortable = false;
                    break;
                }
                prev_max = max;
            }

            if is_sortable {
                res += div;
            }
        }
        
        res as i32
    }
}


impl SolutionOld {
    pub fn sortable_integers(nums: Vec<i32>) -> i32 {
        let common_divisors = common_divisors(nums.len() as i32);
        let mut res = 1;
        let mut nums_is_sorted = true;
        for div in common_divisors.iter().skip(1) {
            let mut n_violations = 0;
            let mut prev = nums[(div - 1) as usize];
            let mut min = 1000000;
            let mut max = 0;
            let mut prev_max = 0;
            
            for (i, n) in nums.iter().enumerate() {
                let idx = i as i32;
                let nb = *n;
                if i != 0 && idx % div == 0 {
                    n_violations = 0;
                    prev = nums[(idx + div - 1) as usize];
                    prev_max = max;
                    max = 0;
                    min = 1000000;
                }

                if prev > nb {
                    // println!("in {div}: {prev}, {nb}");
                    n_violations += 1;

                    if nums_is_sorted && idx % div != 0 {
                        res -= 1;
                        nums_is_sorted = false;
                    }
                }

                if nb < min {
                    min = nb;
                }

                if nb > max {
                    max = nb;
                }

                if n_violations > 1 || min < prev_max {
                    res -= div;
                    break;
                }

                prev = nb;
            }

            // println!("out {div}: {n_violations}, {min}, {max}");
            res += div;
            // println!("out {div}: {res}");
        }

        res
    }
}


pub fn common_divisors(n: i32) -> Vec<i32> {
    if n == 1 {
        return vec![1];
    }
    let limit = n.isqrt() + 1;
    let mut divisors = vec![1, n];
    for div in 2..limit {
        if n % div == 0 {
            divisors.push(div);
            let other = n / div;
            if div != other {
                divisors.push(n / div)
            }
        }
    }
    divisors
}
