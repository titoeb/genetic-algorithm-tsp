use crate::distance_mat::DistanceMat;
use std::collections::HashSet;

/// Get the test matrix that should be use tests.
///
pub fn test_dist_mat() -> DistanceMat {
    // TODO: Either this matrix should be four dimensional
    // or there should also be a four dimensional matrix.
    DistanceMat::new(vec![
        vec![0.0, 1.0, 2.0],
        vec![1.0, 0.0, 3.0],
        vec![2.0, 3.0, 0.0],
    ])
}
/// Test if a sequence is a valid permutation for another sequence.
///
/// Permuation means that it consists exactly all elements from the
/// or original sequence (both have same lenght) but potentially in
/// a different order.
///
/// # Arguments
///
/// * `sequence` - The actual sequence
/// * `permutation` - The potential permutation of the original sequence that should be tested.
///
pub fn valid_permutation(sequence: &[usize], permutation: &[usize]) {
    assert_eq!(sequence.len(), permutation.len());
    assert!(sequence
        .iter()
        .cloned()
        .collect::<HashSet<usize>>()
        .is_superset(&permutation.iter().cloned().collect::<HashSet<usize>>()));
    assert!(permutation
        .iter()
        .cloned()
        .collect::<HashSet<usize>>()
        .is_superset(&sequence.iter().cloned().collect::<HashSet<usize>>()));
}

#[cfg(test)]
mod tests {
    use super::*;
    mod test_valid_permutation {
        use super::*;
        #[test]
        fn same_subsequence() {
            valid_permutation(&vec![1, 2, 3], &vec![1, 2, 3]);
        }
        #[test]
        fn actual_permuation() {
            valid_permutation(&vec![1, 2, 3], &vec![3, 1, 2]);
        }
        #[test]
        #[should_panic]
        fn invalid_permuation_too_many() {
            valid_permutation(&vec![1, 2, 3], &vec![3, 1, 2, 3]);
        }
        #[test]
        #[should_panic]
        fn invalid_permuation_too_few() {
            valid_permutation(&vec![1, 2, 3], &vec![3, 1]);
        }
        #[test]
        #[should_panic]
        fn invalid_permuation_wrong_elem() {
            valid_permutation(&vec![1, 2, 3], &vec![3, 1, 4]);
        }
    }
}
