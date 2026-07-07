use s_1342_Number_of_Steps_to_Reduce_a_Number_to_Zero::Solution;

#[test]
fn test_number_of_steps_zero() {
    assert_eq!(Solution::number_of_steps(0), 0);
}

#[test]
fn test_number_of_steps_simple() {
    // 14 -> 7 -> 6 -> 3 -> 2 -> 1 -> 0 = 6 steps
    assert_eq!(Solution::number_of_steps(14), 6);
}

#[test]
fn test_number_of_steps_power_of_two() {
    // 8 -> 4 -> 2 -> 1 -> 0 = 4 steps
    assert_eq!(Solution::number_of_steps(8), 4);
}
