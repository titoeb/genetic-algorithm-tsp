use crate::solution::Solution;
use crate::utils::get_elem_from_range;
use std::collections::HashSet;
#[derive(Debug)]
pub struct Subsequence {
    pub start_index: usize,
    pub length: usize,
}

impl Subsequence {
    pub fn new(start_index: usize, length: usize) -> Self {
        Subsequence {
            start_index,
            length,
        }
    }
    pub fn random_subsequence(max: usize) -> Self {
        let start_index = get_elem_from_range(0..(max - 2));
        Subsequence {
            start_index,
            length: get_elem_from_range(1..(max - start_index - 1)),
        }
    }
    pub fn get_unique_elems(&self, solution: &Solution) -> HashSet<usize> {
        solution.indexes[self.start_index..(self.start_index + self.length)]
            .iter()
            .copied()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod test_random_subsequence {
        use super::*;
        #[test]
        #[test]
        #[test]
        #[test]
        fn test_max_10() {
            let max_value = 10;
            let random_subsequence = Subsequence::random_subsequence(max_value);
            assert!(random_subsequence.start_index < max_value);
            assert!(random_subsequence.length < max_value - random_subsequence.start_index);
            assert!(random_subsequence.start_index + random_subsequence.length < max_value);
        }
        #[test]
        #[test]
        #[test]
        #[test]
        fn test_max_2() {
            let max_value = 2;
            let random_subsequence = Subsequence::random_subsequence(max_value);
            assert!(random_subsequence.start_index < max_value);
            assert!(random_subsequence.length < max_value - random_subsequence.start_index);
            assert!(random_subsequence.start_index + random_subsequence.length < max_value);
        }
    }
    mod test_get_unique_elems {
        use super::*;
        use crate::solution::Solution;
        use std::collections::HashSet;
        #[test]
        fn test_trivial_range() {
            assert_eq!(
                Subsequence::new(0, 4).get_unique_elems(&Solution::new(vec![1, 2, 3, 4])),
                HashSet::from([1, 2, 3, 4])
            );
        }
        #[test]
        fn test_actual_range() {
            assert_eq!(
                Subsequence::new(1, 2).get_unique_elems(&Solution::new(vec![1, 2, 3, 4])),
                HashSet::from([2, 3])
            );
        }
        #[test]
        fn test_range_len_1() {
            assert_eq!(
                Subsequence::new(1, 1).get_unique_elems(&Solution::new(vec![1, 2, 3, 4])),
                HashSet::from([2])
            );
        }
    }
}
