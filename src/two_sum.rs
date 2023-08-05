fn two_sum_naive(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, i_val) in nums.iter().enumerate() {
        for (j, j_val) in nums.iter().enumerate() {
            if i == j {
                break;
            }
            if i_val + j_val == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![0, 0]
}

fn two_sum_binomial_coefficient(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut outer_enumerator = nums.iter().enumerate();

    // binomial coefficient - still O(n^2), but O((n^2)/2) because each iteration
    // of the other loop reduces the number of inner loop iterations by 1
    while let Some((i, i_val)) = outer_enumerator.next() {
        let mut inner_enumerator = nums[i + 1..].iter().enumerate();
        while let Some((j, j_val)) = inner_enumerator.next() {
            if i_val + j_val == target {
                let mut r = vec![i as i32, (j + i + 1) as i32];
                r.sort();
                return r;
            }
        }
    }

    vec![0, 0]
}

#[test]
fn test_two_sum_case_1() {
    assert_eq!(vec![0, 1], two_sum_binomial_coefficient(vec![2, 7, 11, 15], 9))
}

#[test]
fn test_two_sum_case_2() {
    assert_eq!(vec![1, 2], two_sum_binomial_coefficient(vec![3, 2, 4], 6))
}

#[test]
fn test_two_sum_case_3() {
    assert_eq!(vec![0, 1], two_sum_binomial_coefficient(vec![3, 3], 6))
}
