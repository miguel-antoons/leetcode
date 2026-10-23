// LeetCode problem: 1390. Four Divisors

pub struct Solution;

impl Solution {
    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for num in nums {
            if num < 6 {
                continue;
            }

            let Some(p) = find_prime_factor(num) else {
                continue;
            };

            let m = num / p;

            if
                m == p * p
                || (m != p && is_m_prime(p, m))
            { // if m == p * p --> num is a cube, if m and p are prime and m is not == p --> num has exactly 4 divisors
                res += 1 + p + m + num;
            }
        }
        res
    }
}


fn find_prime_factor(num: i32) -> Option<i32> {
    // find smallest prime factor of num
    if num % 2 == 0 {
        return Some(2);
    }
    
    if num % 3 == 0 {
        return Some(3);
    }
    
    let limit = num.isqrt();
    let mut d = 5;
    while d <= limit {
        if num % d == 0 {
            return Some(d);
        }
        d += 2;
    }

    None
}


fn is_m_prime(p: i32, m: i32) -> bool {
    if m % p == 0 {
        return false;
    }

    let mut d = if p == 2 { 3 } else { p + 2 };
    let limit = m.isqrt();
    while d <= limit {
        if m % d == 0 {
            return false;
        }
        d += 2;
    }
    
    return true
}



// More generic function:
// impl Solution {
//     pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
//         let mut res = 0;
//         for num in nums {
//             if num < 6 {
//                 continue;
//             }
//             let limit = num.isqrt();
//             let mut temp_sum = 1 + num;
//             let mut n_divisors = 2;
//             if limit * limit - num == 0 {
//                 n_divisors -= 1;
//             }
//             for div in 2..limit + 1 {
//                 if num % div == 0 {
//                     n_divisors += 2;
//                     if n_divisors > 4 {
//                         temp_sum = 0;
//                         break
//                     }
//                     temp_sum += num / div + div;
//                 }
//             }
//             if n_divisors == 4 {
//                 res += temp_sum;
//             }
//         }
//         res
//     }
// }
