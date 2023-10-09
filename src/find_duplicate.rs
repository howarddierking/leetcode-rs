// https://leetcode.com/problems/find-the-duplicate-number/?envType=daily-question&envId=2023-09-19
//
// Find the Duplicate Number
//
// Given an array of integers nums containing n + 1 integers where each integer is in the
// range [1, n] inclusive.
//
// There is only one repeated number in nums, return this repeated number.
//
// You must solve the problem without modifying the array nums and uses only constant extra space.
//
// Example 1:
// Input: nums = [1,3,4,2,2]
// Output: 2
//
// Example 2:
// Input: nums = [3,1,3,4,2]
// Output: 3
//
// Constraints:
//
// 1 <= n <= 105
// nums.length == n + 1
// 1 <= nums[i] <= n
//
// All the integers in nums appear only once except for precisely one integer which appears
// two or more times.

use std::collections::HashSet;

pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    // algorithm
    // brute force: O(n^2)
    //  loop num
    //      n = num[i]
    //      loop num
    //          if num[j] = n return n
    // hashset:
    // hs = new hashset
    // loop num
    //      if num[i] is in hs return num[i]
    //      else add num[i] to hashset
    let mut hs: HashSet<i32> = HashSet::new();
    for num in nums {
        if hs.contains(&num){
            return num;
        }
        hs.insert(num);
    }
    0
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn example_1(){
        let input = vec![1,3,4,2,2];
        let expected = 2;
        assert_eq!(expected, find_duplicate(input));
    }

    #[test]
    fn example_2(){
        let input = vec![3,1,3,4,2];
        let expected = 3;
        assert_eq!(expected, find_duplicate(input));
    }
}