use s_412_Fizz_Buzz::Solution;

#[test]
fn test_fizz_buzz_zero() {
    let expected: Vec<String> = vec![];
    assert_eq!(Solution::fizz_buzz(0), expected);
}

#[test]
fn test_fizz_buzz_small() {
    let expected = vec!["1", "2", "Fizz", "4", "Buzz"];
    assert_eq!(Solution::fizz_buzz(5), expected);
}

#[test]
fn test_fizz_buzz_fizz_buzz() {
    let expected = vec!["1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz", "13", "14", "FizzBuzz"];
    assert_eq!(Solution::fizz_buzz(15), expected);
}
