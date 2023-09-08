/// https://leetcode.com/problems/partition-list/

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    Some(Box::new(ListNode::new(4)))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_example_1(){
        let head = [1,4,3,2,5,2];
        let x = 3;

        let expected = [1,2,2,4,3,5];
    }

    fn test_example_2(){
        let head = [2,1];
        let x = 2;

        let expected = [1,2];
    }
}