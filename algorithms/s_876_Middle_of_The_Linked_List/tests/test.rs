use s_876_Middle_of_The_Linked_List::Solution;
use s_876_Middle_of_The_Linked_List::ListNode;

#[test]
fn test_middle_node_single() {
    let head = ListNode::new(1);
    let result = Solution::middle_node(Some(Box::new(head)));
    assert_eq!(result.unwrap().val, 1);
}

#[test]
fn test_middle_node_even() {
    // 1 -> 2 -> 3 -> 4 -> middle is 3
    let mut head = ListNode::new(1);
    head.next = Some(Box::new(ListNode::new(2)));
    head.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
    head.next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));
    
    let result = Solution::middle_node(Some(Box::new(head)));
    assert_eq!(result.unwrap().val, 3);
}

#[test]
fn test_middle_node_odd() {
    // 1 -> 2 -> 3 -> 4 -> 5 -> middle is 3
    let mut head = ListNode::new(1);
    head.next = Some(Box::new(ListNode::new(2)));
    head.next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
    head.next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(4)));
    head.next.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(5)));
    
    let result = Solution::middle_node(Some(Box::new(head)));
    assert_eq!(result.unwrap().val, 3);
}
