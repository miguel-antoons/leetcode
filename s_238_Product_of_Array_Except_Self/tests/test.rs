use s_238_Product_of_Array_Except_Self::Solution;

#[test]
fn test_product_except_self_simple() {
    let nums = vec![1, 2, 3, 4];
    assert_eq!(Solution::product_except_self(nums), vec![24, 12, 8, 6]);
}

#[test]
fn test_product_except_self_with_zero() {
    let nums = vec![0, 1, 2, 3];
    assert_eq!(Solution::product_except_self(nums), vec![6, 0, 0, 0]);
}

#[test]
fn test_product_except_self_single_element() {
    let nums = vec![5];
    assert_eq!(Solution::product_except_self(nums), vec![1]);
}
