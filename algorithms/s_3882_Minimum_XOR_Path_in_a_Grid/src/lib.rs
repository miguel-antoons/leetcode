// LeetCode problem: 3882. Minimum XOR Path in a Grid


pub struct Solution;

#[derive(Clone)]
pub struct BitSet {
    bits: Vec<usize>,
}


impl BitSet {
    fn new(size: usize) -> Self {
        let words = size.div_ceil(usize::BITS as usize);
        Self {
            bits: vec![0usize; words],
        }
    }

    fn insert(&mut self, n: usize) {
        let idx = n / usize::BITS as usize;
        let mask = 1usize << (n % usize::BITS as usize);
        self.bits[idx] |= mask;
    }

    fn clear(&mut self) {
        for i in 0..self.bits.len() {
            self.bits[i] = 0;
        }
    }

    fn get_all(&self) -> Vec<usize> {
        let mut res = Vec::new();
        for (i, &word) in self.bits.iter().enumerate() {
            let mut w = word;
            while w != 0 {
                let local_bit = w.trailing_zeros() as usize;
                res.push(i * usize::BITS as usize + local_bit);
                w &= w - 1;
            }
        }
        res
    }

    fn union(&self, other: &BitSet) -> Vec<usize> {
        let mut res = Vec::new();
        for (i, &word) in self.bits.iter().enumerate() {
            let mut w = word | other.bits[i];
            while w != 0 {
                let local_bit = w.trailing_zeros() as usize;
                res.push(i * usize::BITS as usize + local_bit);
                w &= w - 1;
            }
        }
        
        res
    }

    fn min(&self) -> usize {
        for (i, &word) in self.bits.iter().enumerate() {
            if word == 0 {
                continue;
            }
            let local_bit = word.trailing_zeros() as usize;
            return i * usize::BITS as usize + local_bit;
        }
        return usize::MAX;
    }
}


impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
        let mut rolling_row: Vec<BitSet> = vec![BitSet::new(2047); grid[0].len()];

        let mut current = 0;
        for (idx, col) in grid[0].iter().enumerate() {
            current ^= col;
            rolling_row[idx].insert(current as usize);
        }
        
        for row in 1..grid.len() {
            let intermediate = rolling_row[0].get_all();
            rolling_row[0].clear();
            for prev in intermediate {
                rolling_row[0].insert(grid[row][0] as usize ^ prev);
            }
            for col in 1..grid[row].len() {
                let intermediate: Vec<usize> = rolling_row[col].union(&rolling_row[col - 1]);
                rolling_row[col].clear();
                for i in intermediate {
                    rolling_row[col].insert(grid[row][col] as usize ^ i);
                }
            }
        }
        return rolling_row[rolling_row.len() - 1].min() as i32;
    }
}

// impl Solution {
//     pub fn min_cost(grid: Vec<Vec<i32>>) -> i32 {
//         let mut rolling_row: Vec<HashSet<i32>> = vec![HashSet::new(); grid[0].len()];
// 
//         let mut current = 0;
//         for (idx, col) in grid[0].iter().enumerate() {
//             current ^= col;
//             rolling_row[idx].insert(current);
//         }
//         
//         for row in 1..grid.len() {
//             // rolling_row[0] = rolling_row[0] ^ grid[row][0];
//             let intermediate: Vec<i32> = rolling_row[0].iter().copied().collect();
//             rolling_row[0].clear();
//             for prev in intermediate {
//                 rolling_row[0].insert(grid[row][0] ^ prev);
//             }
//             for col in 1..grid[row].len() {
//                 let intermediate: Vec<i32> = rolling_row[col].union(&rolling_row[col - 1]).into_iter().copied().collect();
//                 rolling_row[col].clear();
//                 for i in intermediate {
//                     rolling_row[col].insert(grid[row][col] ^ i);
//                 }
//             }
//         }
//         return *rolling_row[rolling_row.len() - 1].iter().min().unwrap();
//     }
// }
