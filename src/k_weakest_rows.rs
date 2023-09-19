use std::cmp::Ordering;

/// https://leetcode.com/problems/the-k-weakest-rows-in-a-matrix/?envType=daily-question&envId=2023-09-18
///
/// You are given an m x n binary matrix mat of 1's (representing soldiers) and 0's (representing
/// civilians). The soldiers are positioned in front of the civilians. That is, all the 1's will
/// appear to the left of all the 0's in each row.
///
/// A row i is weaker than a row j if one of the following is true:
///
/// * The number of soldiers in row i is less than the number of soldiers in row j.
/// * Both rows have the same number of soldiers and i < j.
/// * Return the indices of the k weakest rows in the matrix ordered from weakest to strongest.

fn k_weakest_rows(mat: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
    // map the matrix to a new vector consisting of
    // - index
    // - sum of the interior vector
    // sort the mapped vector based on the above rules - this will be ascending based on count of
    //  1s plus row index in the case where the count is equal
    // take k

    let mut m: Vec<(usize, i32)> = mat.iter().enumerate().map(|(i, row)|(i, row.iter().sum::<i32>())).collect();
    m.sort_by(|a, b|{
        if a.1 > b.1 {
            return Ordering::Greater
        }
        if a.1 == b.1 && a.0 > b.0 {
            return Ordering::Greater
        }
        Ordering::Less
    });
    m.iter().take(k as usize).map(|(i, c)| *i as i32).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1(){
        let matrix = vec![
            vec![1,1,0,0,0],
            vec![1,1,1,1,0],
            vec![1,0,0,0,0],
            vec![1,1,0,0,0],
            vec![1,1,1,1,1],
        ];

        let expected = vec![2,0,3];
        let k = 3;
        assert_eq!(expected, k_weakest_rows(matrix, k));
    }

    #[test]
    fn example_2(){
        todo!()
    }
}