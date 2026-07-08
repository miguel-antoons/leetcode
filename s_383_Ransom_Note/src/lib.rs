// LeetCode problem: 383. Ransom Note

pub struct Solution;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let offset = 'a' as usize;
        let mut chars: [u16; 26] = Default::default();
        for c in magazine.chars() {
            chars[(c as usize) - offset] += 1;
        }

        for c in ransom_note.chars() {
            if chars[(c as usize) - offset] == 0 {
                return false;
            }
            chars[(c as usize) - offset] -= 1;
        }
        
        return true;
    }
}
