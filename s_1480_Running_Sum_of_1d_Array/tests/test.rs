use s_1480_Running_Sum_of_1d_Array::Solution;

#[test]
fn test_running_sum_empty() {
    let nums: Vec<i32> = vec![];
    assert_eq!(Solution::running_sum(nums), vec![]);
}

#[test]
fn test_running_sum_simple() {
    let nums = vec![1, 2, 3, 4];
    assert_eq!(Solution::running_sum(nums), vec![1, 3, 6, 10]);
}

#[test]
fn test_running_sum_single_element() {
    let nums = vec![5];
    assert_eq!(Solution::running_sum(nums), vec![5]);
}
