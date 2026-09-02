// LeetCode problem: 876. Middle of The Linked List

pub struct Solution;


// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}


impl ListNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}


impl Solution {
    pub fn middle_node(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut next = head.as_ref();
        let mut n = 0;

        while let Some(node) = next {
            next = node.next.as_ref();
            n += 1;
        }

        n /= 2;
        for _ in 0..n {
            head = head.and_then(|n| n.next);
        }

        return head;
    }
}
