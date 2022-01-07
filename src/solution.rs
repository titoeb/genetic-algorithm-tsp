use crate::distance_mat::DistanceMat;
use crate::subsequence::Subsequence;
use crate::utils::{change_order, get_elem_from_range, ordered_crossover, remove_elem};
use rand::seq::SliceRandom;

use std::cmp::max;
#[derive(Debug, PartialEq, Clone)]
pub struct Solution {
    pub indexes: Vec<usize>,
}
impl Solution {
    pub fn new(indexes: Vec<usize>) -> Self {
        Self { indexes }
    }
    pub fn mutate(self, prob: f32) -> Self {
        Solution {
            indexes: if get_elem_from_range(0.0..1.0) > prob {
                self.indexes
            } else {
                let put_before_idx: usize = get_elem_from_range(0..(self.indexes.len() - 1));
                change_order(
                    &self.indexes,
                    put_before_idx,
                    *remove_elem(
                        remove_elem(
                            (0..(self.indexes.len() - 1)).collect::<Vec<usize>>(),
                            &put_before_idx,
                        ),
                        &(max(put_before_idx, 1) - 1),
                    )
                    .choose(&mut rand::thread_rng())
                    .unwrap_or(&((put_before_idx + 1) % self.indexes.len())),
                )
            },
        }
    }
    pub fn crossover(&self, other: &Solution) -> Self {
        ordered_crossover(
            self,
            other,
            Subsequence::random_subsequence(self.indexes.len()),
        )
    }
    pub fn fitness(&self, distance_mat: &DistanceMat) -> f64 {
        distance_mat.get_distance(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod test_solution {
        use super::*;
        use crate::test_utils::valid_permutation;
        #[test]
        fn test_constructor() {
            let solution = Solution::new(vec![1, 2, 3, 4]);
            assert_eq!(solution.indexes, vec![1, 2, 3, 4])
        }
        #[test]
        fn test_mutuate_no_prob() {
            assert_eq!(
                Solution::new(vec![1, 2, 3, 4]).mutate(0.0).indexes,
                vec![1, 2, 3, 4]
            )
        }
        // Run the following test five times.
        #[test]
        #[test]
        #[test]
        #[test]
        #[test]
        fn test_mutuate_100_prob() {
            assert_ne!(
                Solution::new(vec![1, 2, 3, 4]).mutate(1.0).indexes,
                vec![1, 2, 3, 4]
            )
        }
        #[test]
        fn test_mutuate_100_prob_3_elems() {
            assert_ne!(
                Solution::new(vec![1, 2, 3]).mutate(1.0).indexes,
                vec![1, 2, 3]
            )
        }
        #[test]
        fn test_mutate_simple_run() {
            let test_solution = Solution::new(vec![1, 2, 0]);
            valid_permutation(
                &test_solution.indexes,
                &test_solution.clone().mutate(0.5).indexes,
            );
        }
    }
    mod test_crossover {
        use super::*;
        use crate::test_utils::valid_permutation;

        #[test]
        fn random_test_10() {
            let n_tests = 1000;
            let solution_a = Solution {
                indexes: vec![0, 12, 7, 3, 9, 8, 11, 5, 13, 1, 4, 6, 10, 15, 2, 14],
            };
            let solution_b = Solution {
                indexes: vec![7, 10, 15, 12, 2, 9, 5, 3, 1, 6, 4, 13, 14, 11, 8, 0],
            };
            let mut n_no_crossover = 0;
            for _ in 1..n_tests {
                let result = solution_a.crossover(&solution_b);
                if result.indexes == solution_a.indexes || result.indexes == solution_b.indexes {
                    n_no_crossover += 1;
                }
                valid_permutation(&result.indexes, &solution_a.indexes);
                valid_permutation(&result.indexes, &solution_a.indexes);
            }
            assert!(n_no_crossover <= n_tests / 5);
        }
    }
    mod test_fitness {
        use super::*;
        use crate::test_utils::test_dist_mat;
        #[test]
        fn simple_functionality_test() {
            let distance_mat = test_dist_mat();
            let solution = Solution::new(vec![1, 2, 0]);
            assert_eq!(solution.fitness(&distance_mat), 6.0);
        }
    }
}
