/// https://leetcode.com/problems/search-a-2d-matrix/
///
/// The key constraint for the problem is that it should run in O(log(n*m)). Since, the problem
/// effectively calls for 2 layers of searches (different comparisons, but same idea), and since
/// both dimensions of the search are effectively sorted vectors, we should be able to meet this
/// constraint by implementing a [binary search](https://en.wikipedia.org/wiki/Binary_search_algorithm)
/// for both outer and inner loops.
///
/// to be as idiomatically Rusty as possible, we should implement an iterator for traversing the
/// vectors.
pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let rowResult = matrix.binary_search_by(|probe|{
        probe.get(0).unwrap().cmp(&target)
    });

    let rowIdx = match rowResult{
        Ok(i) => { i }
        Err(i) => { i - 1 }
    };

    if let Some(row) = matrix.get(rowIdx) {
        let colResult = row.binary_search(&target);
        if colResult.is_ok() {
            return true;
        }
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_find_input() {
        let expected = true;
        let input = vec!(
            vec!(1,3,5,7),
            vec!(10,11,16,20),
            vec!(23,30,34,60),
        );
        let actual = search_matrix(input, 3);
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_not_find_input(){
        let expected = false;
        let input = vec!(
            vec!(1,3,5,7),
            vec!(10,11,16,20),
            vec!(23,30,34,60),
        );
        let actual = search_matrix(input, 13);
        assert_eq!(expected, actual);
    }
}