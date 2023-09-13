use std::ops::Rem;

/// https://leetcode.com/problems/add-two-numbers/
///
/// You are given two non-empty linked lists representing two non-negative integers. The digits
/// are stored in reverse order, and each of their nodes contains a single digit. Add the two
/// numbers and return the sum as a linked list.
///
/// You may assume the two numbers do not contain any leading zero, except the number 0 itself.

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
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

fn rev_listnode_to_i32(mut head: Option<&Box<ListNode>>) -> i32 {
    let mut i = 0;
    let mut r = 0;
    while let Some(h) = head{
        r += h.val * 10_i32.pow(i);
        i+=1;
        head = h.next.as_ref();
    }
    r
}

fn i32_to_rev_listnode(val: i32) -> Option<Box<ListNode>> {
    let mut i = 1;
    let mut place_value = val;

    let mut ret = Some(Box::new(ListNode::new(place_value.rem(10))));
    let mut head = &mut ret;

    // TODO: clean up logic here

    // loop until integer division produces zero
    while place_value > 0 {
        place_value = val / 10_i32.pow(i);
        if place_value == 0 {
            return ret;
        }
        let remainder = place_value.rem(10);
        let new_node = Some(Box::new(ListNode::new(remainder)));

        let node = head.as_mut().unwrap();
        node.next = new_node;
        head = &mut node.next;

        i+=1;
    }

    ret
}


fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // algorithm
    // for each list, produce an int by traversing the list and adding (val*10^i) where i starts at 0
    // add the ints
    let sum = rev_listnode_to_i32(l1.as_ref()) + rev_listnode_to_i32(l2.as_ref());

    // create a new linked list (reverse order) by looping on (val / 10^i) % 10
    i32_to_rev_listnode(sum)
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_rev_listnode_to_i32(){
        let mut l1 = ListNode::new(2);
        let mut l2 = ListNode::new(4);
        let mut l3 = ListNode::new(3);

        l2.next = Some(Box::new(l3));
        l1.next = Some(Box::new(l2));

        let actual = rev_listnode_to_i32(Some(Box::new(l1)).as_ref());

        assert_eq!(342, actual)
    }

    #[test]
    fn test_i32_to_rev_listnode(){
        let mut l1 = ListNode::new(2);
        let mut l2 = ListNode::new(4);
        let mut l3 = ListNode::new(3);

        l2.next = Some(Box::new(l3));
        l1.next = Some(Box::new(l2));

        let expected = Some(Box::new(l1));
        let actual = i32_to_rev_listnode(342);
        assert_eq!(expected, actual)
    }

    #[test]
    fn example_1(){
        // Input: l1 = [2,4,3], l2 = [5,6,4]
        // Output: [7,0,8]
        // Explanation: 342 + 465 = 807.
        todo!()
    }

    #[test]
    fn example_2(){
        // Input: l1 = [0], l2 = [0]
        // Output: [0]
        todo!()
    }

    #[test]
    fn example_3(){
        // Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
        // Output: [8,9,9,9,0,0,0,1]
        todo!()
    }
}