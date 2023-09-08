/// https://leetcode.com/problems/implement-stack-using-queues/
///
/// The solution here is maybe simpler than the problem would have liked because Rust std
/// collections includes VecDeque (double-entry queue vector). The data structure is implemented
/// as ring buffer (in addition to buffer ptr and size, the struct also includes start and stop
/// pointers to members of the larger buffer). This means that stack operations like push and pop
/// simply move the start pointer and don't require shifting elements - and as a result of that
/// capability, the structure already supports stack functions and this challenge simply needs
/// to delegate to those.

use std::collections::VecDeque;

struct MyStack {
    q: VecDeque<i32>
}

/**
  * `&self` means the method takes an immutable reference
  * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyStack {

    fn new() -> Self {
        Self{
            q: VecDeque::new()
        }
    }

    fn push(&mut self, x: i32) {
        self.q.push_front(x)
    }

    fn pop(&mut self) -> i32 {
        self.q.pop_front().unwrap()
    }

    fn top(&self) -> i32 {
        *self.q.front().unwrap()
    }

    fn empty(&self) -> bool {
        self.q.is_empty()
    }
}

/**
 * Your MyStack object will be instantiated and called as such:
 * let obj = MyStack::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: bool = obj.empty();
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1(){
        // Input
        // ["MyStack", "push", "push", "top", "pop", "empty"]
        // [[], [1], [2], [], [], []]
        // Output
        // [null, null, null, 2, 2, false]
        //
        // Explanation
        let mut myStack = MyStack::new();
        myStack.push(1);
        myStack.push(2);

        assert_eq!(2, myStack.top());
        assert_eq!(2, myStack.pop());
        assert_eq!(false, myStack.empty());
    }
}