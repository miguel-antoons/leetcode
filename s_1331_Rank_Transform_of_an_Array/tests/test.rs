use s_1331_Rank_Transform_of_an_Array::Solution;

#[test]
fn test_empty_array() {
    let input = vec![];
    let expected = vec![];
    assert_eq!(Solution::array_rank_transform(input), expected);
}

#[test]
fn test_with_duplicates() {
    let input = vec![10, 20, 20, 30];
    let expected = vec![1, 2, 2, 3];
    assert_eq!(Solution::array_rank_transform(input), expected);
}

#[test]
fn test_all_same() {
    let input = vec![5, 5, 5, 5];
    let expected = vec![1, 1, 1, 1];
    assert_eq!(Solution::array_rank_transform(input), expected);
}

#[test]
fn test_unique_unsorted() {
    let input = vec![40, 10, 20, 30];
    let expected = vec![4, 1, 2, 3];
    assert_eq!(Solution::array_rank_transform(input), expected);
}

#[test]
fn test_single_element() {
    let input = vec![100];
    let expected = vec![1];
    assert_eq!(Solution::array_rank_transform(input), expected);
}
