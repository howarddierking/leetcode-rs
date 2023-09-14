use std::ops::Rem;

/// https://leetcode.com/problems/add-two-numbers/
///
/// You are given two non-empty linked lists representing two non-negative integers. The digits
/// are stored in reverse order, and each of their nodes contains a single digit. Add the two
/// numbers and return the sum as a linked list.
///
/// You may assume the two numbers do not contain any leading zero, except the number 0 itself.
///
/// UPDATE: well, I thought I had a good algorithm, but it turns out that the true nature of the
/// problem is about summing 2 numbers that would overflow a normal int data type

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

fn rev_listnode_to_int(mut head: Option<&Box<ListNode>>) -> i64 {
    let mut i = 0;
    let mut r: i64 = 0;
    while let Some(h) = head{
        r += h.val as i64 * 10_i64.pow(i);
        i+=1;
        head = h.next.as_ref();
    }
    r
}

fn int_to_rev_listnode(val: i64) -> Option<Box<ListNode>> {

    let r = val.rem(10);
    let v = val / 10;

    let mut node = Box::new(ListNode::new(r as i32));

    if v > 0 {
        node.next = int_to_rev_listnode(v);
    }

    Some(node)
}


fn add_two_numbers_OLD(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // algorithm
    // for each list, produce an int by traversing the list and adding (val*10^i) where i starts at 0
    // add the ints
    let sum = rev_listnode_to_int(l1.as_ref()) + rev_listnode_to_int(l2.as_ref());

    // create a new linked list (reverse order) by looping on (val / 10^i) % 10
    int_to_rev_listnode(sum)
}

fn add_two_numbers_with_carry(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, carry: i32) -> Option<Box<ListNode>> {
    match(l1, l2){
        (Some(a), Some(b)) => {
            let sum = a.val + b.val + carry;
            let mut temp = ListNode::new(sum.rem(10));
            temp.next = add_two_numbers_with_carry(a.next, b.next, sum / 10);
            return Some(Box::new(temp));
        },
        (Some(a), None) => {
            let sum = a.val + carry;
            let mut temp = ListNode::new(sum.rem(10));
            temp.next = add_two_numbers_with_carry(a.next, None, sum / 10);
            return Some(Box::new(temp));
        },
        (None, Some(b)) => {
            let sum = b.val + carry;
            let mut temp = ListNode::new(sum.rem(10));
            temp.next = add_two_numbers_with_carry(None, b.next, sum / 10);
            return Some(Box::new(temp));
        },
        (None, None) => {
            if carry > 0 {
                return Some(Box::new(ListNode::new(carry)));
            } else {
                return None;
            }
        }
    }
}


fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    add_two_numbers_with_carry(l1, l2, 0)
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_rev_listnode_to_i64(){
        let l =
            Some(Box::new(ListNode{val: 2, next:
                Some(Box::new(ListNode{val: 4, next:
                    Some(Box::new(ListNode{val: 3, next: None}))
                }))
            }));

        let actual = rev_listnode_to_int(l.as_ref());

        assert_eq!(342, actual)
    }

    #[test]
    fn test_i64_to_rev_listnode(){
        let expected =
            Some(Box::new(ListNode{val: 2, next:
                Some(Box::new(ListNode{val: 4, next:
                    Some(Box::new(ListNode{val: 3, next: None}))
                }))
            }));

        let actual = int_to_rev_listnode(342);
        assert_eq!(expected, actual)
    }

    #[test]
    fn example_1(){
        // l1 = [2,4,3]
        // l2 = [5,6,4]
        // Output: [7,0,8]
        // Explanation: 342 + 465 = 807.

        let l1 =
            Some(Box::new(ListNode{val: 2, next:
                Some(Box::new(ListNode{val: 4, next:
                    Some(Box::new(ListNode{val: 3, next: None}))
                }))
            }));

        let l2 =
            Some(Box::new(ListNode{val: 5, next:
                Some(Box::new(ListNode{val: 6, next:
                    Some(Box::new(ListNode{val: 4, next: None}))
                }))
            }));

        let expected =
            Some(Box::new(ListNode{val: 7, next:
                Some(Box::new(ListNode{val: 0, next:
                    Some(Box::new(ListNode{val: 8, next: None}))
                }))
            }));

        assert_eq!(expected, add_two_numbers(l1, l2));
    }

    #[test]
    fn example_2(){
        // Input: l1 = [0]
        // l2 = [0]
        // Output: [0]

        let expected = Some(Box::new(ListNode::new(0)));
        let actual = add_two_numbers(
            Some(Box::new(ListNode::new(0))),
            Some(Box::new(ListNode::new(0))));

        assert_eq!(expected, actual);
    }

    #[test]
    fn example_3(){
        // l1 = [9,9,9,9,9,9,9]
        // l2 = [9,9,9,9]
        // Output: [8,9,9,9,0,0,0,1]
        let l1 =
            Some(Box::new(ListNode{val: 9, next:
                Some(Box::new(ListNode{val: 9, next:
                    Some(Box::new(ListNode{val: 9, next:
                        Some(Box::new(ListNode{val: 9, next:
                            Some(Box::new(ListNode{val: 9, next:
                                Some(Box::new(ListNode{val: 9, next:
                                    Some(Box::new(ListNode{val: 9, next: None}))
                                }))
                            }))
                        }))
                    }))
                }))
            }));

        let l2 =
            Some(Box::new(ListNode{val: 9, next:
                Some(Box::new(ListNode{val: 9, next:
                    Some(Box::new(ListNode{val: 9, next:
                        Some(Box::new(ListNode{val: 9, next: None}))
                    }))
                }))
            }));

        let expected =
            Some(Box::new(ListNode{val: 8, next:
                Some(Box::new(ListNode{val: 9, next:
                    Some(Box::new(ListNode{val: 9, next:
                        Some(Box::new(ListNode{val: 9, next:
                            Some(Box::new(ListNode{val: 0, next:
                                Some(Box::new(ListNode{val: 0, next:
                                    Some(Box::new(ListNode{val: 0, next:
                                        Some(Box::new(ListNode{val: 1, next: None}))
                                    }))
                                }))
                            }))
                        }))
                    }))
                }))
            }));

        assert_eq!(expected, add_two_numbers(l1, l2));
    }

    #[test]
    fn example_4(){
        // l1 = [9]
        // l2 = [1,9,9,9,9,9,9,9,9,9]
        // Expected = [0,0,0,0,0,0,0,0,0,0,1]

        let l1 = Some(Box::new(ListNode::new(9)));

        let l2 =
            Some(Box::new(ListNode{ val: 1, next:
                Some(Box::new(ListNode{ val: 9, next:
                    Some(Box::new(ListNode{ val: 9, next:
                        Some(Box::new(ListNode{ val: 9, next:
                            Some(Box::new(ListNode{ val: 9, next:
                                Some(Box::new(ListNode{ val: 9, next:
                                    Some(Box::new(ListNode{ val: 9, next:
                                        Some(Box::new(ListNode{ val: 9, next:
                                            Some(Box::new(ListNode{ val: 9, next:
                                                Some(Box::new(ListNode{ val: 9, next: None }))
                                            }))
                                        }))
                                    }))
                                }))
                            }))
                        }))
                    }))
                }))
            }));

        let expected =
            Some(Box::new(ListNode{ val: 0, next:
                Some(Box::new(ListNode{ val: 0, next:
                    Some(Box::new(ListNode{ val: 0, next:
                        Some(Box::new(ListNode{ val: 0, next:
                            Some(Box::new(ListNode{ val: 0, next:
                                Some(Box::new(ListNode{ val: 0, next:
                                    Some(Box::new(ListNode{ val: 0, next:
                                        Some(Box::new(ListNode{ val: 0, next:
                                            Some(Box::new(ListNode{ val: 0, next:
                                                Some(Box::new(ListNode{ val: 0, next:
                                                    Some(Box::new(ListNode{ val: 1, next: None }))
                                                }))
                                            }))
                                        }))
                                    }))
                                }))
                            }))
                        }))
                    }))
                }))
            }));

        assert_eq!(expected, add_two_numbers(l1, l2));
    }
}