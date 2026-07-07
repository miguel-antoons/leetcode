use s_1672_Richest_Customer_Wealth::Solution;

#[test]
fn test_maximum_wealth_single_customer() {
    let accounts = vec![vec![1, 2, 3]];
    assert_eq!(Solution::maximum_wealth(accounts), 6);
}

#[test]
fn test_maximum_wealth_multiple_customers() {
    let accounts = vec![vec![1, 5], vec![7, 3], vec![3, 5]];
    assert_eq!(Solution::maximum_wealth(accounts), 10);
}

#[test]
fn test_maximum_wealth_empty_accounts() {
    let accounts = vec![vec![], vec![1, 2]];
    assert_eq!(Solution::maximum_wealth(accounts), 3);
}
