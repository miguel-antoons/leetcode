use s_383_Ransom_Note::Solution;

#[test]
fn test_can_construct_true() {
    assert!(Solution::can_construct("aa".to_string(), "aab".to_string()));
}

#[test]
fn test_can_construct_false() {
    assert!(!Solution::can_construct("aa".to_string(), "ab".to_string()));
}

#[test]
fn test_empty_ransom_note() {
    assert!(Solution::can_construct("".to_string(), "abc".to_string()));
}
