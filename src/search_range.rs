// 34. Find First and Last Position of Element in Sorted Array
//
// Given an array of integers nums sorted in non-decreasing order, find the starting and ending
// position of a given target value.
//
// If target is not found in the array, return [-1, -1].
//
// You must write an algorithm with O(log n) runtime complexity.
//
// Constraints:
//
// 0 <= nums.length <= 105
// -109 <= nums[i] <= 109
// nums is a non-decreasing array.
// -109 <= target <= 109

fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // binary search
    // walk forward until item is not target
    let r = nums.binary_search(&target);
    match r {
        Ok(i) => {
            // search backwards for first
            let mut first = i;
            // aaaaaand this is why you should use iterators for everything
            while first > 0 && nums[first - 1] == target {
                first -= 1;
            }

            // search forwards for last
            let mut last = i;
            while last+1 < nums.len() && nums[last + 1] == target {
                last += 1;
            }

            return vec![first as i32, last as i32];
        }
        Err(_) => {
            return vec![-1, -1];
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn example_1(){
        // Input: nums = [5,7,7,8,8,10], target = 8
        // Output: [3,4]

        assert_eq!(vec![3,4], search_range(vec![5,7,7,8,8,10], 8));
    }

    #[test]
    fn example_2(){
        // Input: nums = [5,7,7,8,8,10], target = 6
        // Output: [-1,-1]
        assert_eq!(vec![-1,-1], search_range(vec![5,7,7,8,8,10], 6));
    }

    #[test]
    fn example_3(){
        // Input: nums = [], target = 0
        // Output: [-1,-1]
        assert_eq!(vec![-1,-1], search_range(vec![], 0));
    }

    #[test]
    fn submission_error_1(){
        assert_eq!(vec![0,0], search_range(vec![1], 1));
    }
}