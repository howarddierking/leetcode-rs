/// https://leetcode.com/problems/split-linked-list-in-parts/?envType=daily-question&envId=2023-09-06
///
/// Split Linked List in Parts
///
/// Given the head of a singly linked list and an integer k, split the linked list into k
/// consecutive linked list parts.
///
/// The length of each part should be as equal as possible: no two parts should have a size
/// differing by more than one. This may lead to some parts being null.
///
/// The parts should be in the order of occurrence in the input list, and parts occurring
/// earlier should always have a size greater than or equal to parts occurring later.
///
/// Return an array of the k parts.
///
/// UPDATE:
/// I have an algorithm that I think will work, but I'm continually beating my head against the
/// wall trying to work well with Rust's borrow checker, since Option<T> requires move a move
/// every time the value is used. I'll list the algorithm here and then look at existing solutions
/// so as to not get stuck
///
/// Algorithm:
/// find length of the list
/// find number of groups by integer division of length and k
/// find remainder by modulo length and k
/// calculate size of first group (k + 1 if remainder > 0; decrement remainder)
/// read to tail of group
/// break into LHS, RHS
/// add LHS to output vector
/// while RHS is_some
///     calculate size of next group
///     read to tail of group
///     break into LHS, RHS
///     add LHS to output vector

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

// NOTE: remember that you don't want to consume head here as we need to walk the list later
fn count_linked_list(head: &Option<Box<ListNode>>) -> i32 {
    match head {
        None => 0,
        Some(n) => 1+ count_linked_list(&n.next)
    }
}

pub fn BROKEN_split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
    // count how many items are in the LinkedList so that we know how many partitions to make
    let list_length = count_linked_list(&head);

    // quick exit if list length is 0
    if list_length == 0 {
        return vec![None; k as usize];
    }

    // quick(ish) exit if length is less than or equal to k;
    // each item is a member of the vector with None next pointers and None as the pad for remainders
    if list_length <= k {
        // for the range of 0..k
        //  if head is Some<_>
        //      temp = head.clone
        //      temp.next = None
        //      head = head.next
        //      vec push Some(temp)
        //  else
        //      vec push None

        let mut ret = Vec::new();
        let mut h = head;

        for i in 0..k {
            if h.is_none(){
                ret.push(None);
            } else {
                let mut temp_node = h.clone().unwrap();
                temp_node.next = None;
                h = h.unwrap().next;
                ret.push(Some(temp_node));
            }
        }

        return ret;
    }

    // calculate how many groups we need and how many of those groups we need to
    // distribute a remainder over
    let mut remainder = list_length % k;
    let groups = list_length / k;
    let mut position = 1;
    let mut ret: Vec<Option<Box<ListNode>>> = Vec::new();

    let mut group_size = k;
    if remainder > 0 {
        remainder -= 1;
        group_size += 1;
    }

    // for i in 0..list_length {
    //     if i < group_size
    // }

    todo!()
}

fn split_off(mut val: &Option<Box<ListNode>>, position: i32) -> Option<Box<ListNode>> {
    if val.is_none(){
        return None;
    }
    // read to position - that position will be tail for the split
    // if you don't get to position, return None and do not mutate val
    // if node at position is the last item in the linked list, return None and do not mutate val
    let mut visited = val.as_ref().unwrap();
    for i in 0..position {
        if visited.next.is_some(){
            visited = visited.next.as_ref().unwrap();
        } else {
            return None;
        }
    }

    if visited.next.is_none() {
        return None;
    }

    // let temp = visited.next;
    // val = &temp;

    todo!()
}


/// working solution
/// https://leetcode.com/problems/split-linked-list-in-parts/solutions/4011651/in-place-split/?envType=daily-question&envId=2023-09-06
///

fn get_len(mut head: Option<&Box<ListNode>>) -> usize {
    let mut len = 0;
    while let Some(node) = head {
        head = node.next.as_ref();
        len += 1;
    }
    len
}

fn split(mut head: Option<&mut Box<ListNode>>, mut len: usize) -> Option<Box<ListNode>> {
    while let Some(node) = head {
        len -= 1; if len == 0 { return node.next.take() }
        head = node.next.as_mut()
    }
    None
}


pub fn split_list_to_parts(mut head: Option<Box<ListNode>>, mut k: i32) -> Vec<Option<Box<ListNode>>> {
    let mut len = get_len(head.as_ref());
    let mut ans = Vec::new();

    while k > 0 {
        let take = len / k as usize + (len % k as usize != 0) as usize;
        let tail = split(head.as_mut(), take);
        ans.push(head);
        head = tail;
        len -= take;
        k -= 1;
    }
    ans
}

/// end working solution






#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn count_empty(){
        assert_eq!(0, count_linked_list(&None::<Box<ListNode>>))
    }

    #[test]
    fn count_three(){
        let mut one = Box::new(ListNode::new(1));
        let mut two = Box::new(ListNode::new(2));
        let three = Box::new(ListNode::new(3));

        two.next = Some(three);
        one.next = Some(two);

        assert_eq!(3, count_linked_list(&Some(one)));
    }

    #[test]
    fn example_1(){
        // Input: head = [1,2,3], k = 5
        // Output: [[1],[2],[3],[],[]]
        // Explanation:
        //     The first element output[0] has output[0].val = 1, output[0].next = null.
        //     The last element output[4] is null, but its string representation as a ListNode is [].

        let k = 5;

        // Q: why do I need to box ListNode? The size can be easily predetermined
        let mut one = Box::new(ListNode::new(1));
        let mut two = Box::new(ListNode::new(2));
        let three = Box::new(ListNode::new(3));

        // you have to build the linked list in reverse because constructing the Option<T> moves
        // the value being wrapped
        two.next = Some(three);
        one.next = Some(two);

        let actual = split_list_to_parts(Some(one), k);
        let mut expected = Vec::new();
        expected.push(Some(Box::new(ListNode::new(1))));
        expected.push(Some(Box::new(ListNode::new(2))));
        expected.push(Some(Box::new(ListNode::new(3))));
        expected.push(None);
        expected.push(None);

        assert_eq!(expected, actual);
    }

    #[test]
    fn example_2(){
        // Input: head = [1,2,3,4,5,6,7,8,9,10], k = 3
        // Output: [[1,2,3,4],[5,6,7],[8,9,10]]
        // Explanation:
        //     The input has been split into consecutive parts with size difference at most 1, and
        //      earlier parts are a larger size than the later parts.
    }

    // example 3
    // Input: head = [1,2,3,4,5,6,7,8,9,10,11], k = 3
    // Output: [[1,2,3,4],[5,6,7,8],[9,10,11]]
    // buket_size
    // bucket_position

}