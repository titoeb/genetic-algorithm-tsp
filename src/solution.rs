use crate::distance_mat::DistanceMat;
use crate::gen_traits::Individual;
use crate::subsequence::Subsequence;
use crate::utils::{change_order, get_random_elem_from_range, ordered_crossover, remove_elem};
use rand::seq::SliceRandom;
use std::cmp::max;

/// The `Solution` is the individual for using generic algorithms to solve Traveling-Salesman-Problems.
#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct Solution {
    /// The order in which the nodes should be visited.
    pub indexes: Vec<usize>,
}
impl Solution {
    /// Create a new solution based on a vector of indexes.
    ///
    /// # Arguments
    ///
    /// * `indexes` - The order in which the nodes are visited in the Traveling Salesman Problem.
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algo::solution::Solution;
    ///
    /// let my_individual = Solution::from(Solution::new(vec![0,1,2]));
    /// ```
    pub fn new(indexes: Vec<usize>) -> Self {
        Self { indexes }
    }
}
impl<'a> Individual<'a> for Solution {
    type IndividualCost = DistanceMat;
    /// Randomly changes the order of two nodes in the solution
    ///
    /// # Arguments
    ///
    /// * `prob` - The probability with which the indexes will be changed
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algo::solution::Solution;
    /// use genetic_algo::gen_traits::Individual;
    ///
    /// let my_individual = Solution::from(Solution::new(vec![0,1,2]));
    /// let my_mutated_indiviual =  my_individual.mutate(1.0);
    /// ```
    fn mutate(self, prob: f32) -> Self {
        Solution {
            indexes: if get_random_elem_from_range(0.0..1.0) > prob {
                // With probabilty (1-prop) don't do any mutation.
                self.indexes
            } else {
                // else mutation is applied.
                // To do so first sample an element to put another element in front of.
                let put_before_idx: usize = get_random_elem_from_range(0..(self.indexes.len() - 1));
                change_order(
                    &self.indexes,
                    put_before_idx,
                    // Sample the element that should be put before `put_before_idx`. Should not be
                    // the `put_before_idx` itself.
                    *remove_elem(
                        remove_elem(
                            (0..(self.indexes.len() - 1)).collect::<Vec<usize>>(),
                            put_before_idx,
                        ),
                        max(put_before_idx, 1) - 1,
                    )
                    .choose(&mut rand::thread_rng())
                    .unwrap_or(&((put_before_idx + 1) % self.indexes.len())),
                )
            },
        }
    }
    /// Crossover this invidual with another individual to create a new individual. Currently
    /// uses the `ordered_crossover` algorithm.
    ///
    /// # Arguments
    ///
    /// * `other` - The other individual you would like to use in the crossover individual.
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algo::solution::Solution;
    /// use genetic_algo::gen_traits::Individual;
    ///
    /// let my_individual = Solution::from(Solution::new(vec![0,1,2]));
    /// let my_individual = my_individual.crossover(
    ///     &Solution::from(Solution::new(vec![1,0,2]))
    /// );
    /// ```
    fn crossover(&self, other: &Solution) -> Self {
        ordered_crossover(
            self,
            other,
            Subsequence::random_subsequence(self.indexes.len()),
        )
    }
    /// Compute how much distance the individual implies with its order of nodes
    /// and the distance matrix.
    ///
    /// # Arguments
    ///
    /// * `distance_matrix` - Distance Matrix that determines the length of the proposed
    /// solution
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algo::solution::Solution;
    /// use genetic_algo::distance_mat::DistanceMat;
    /// use genetic_algo::gen_traits::Individual;
    ///
    /// let my_individual = Solution::from(Solution::new(vec![0,1,2]));
    /// println!("Fitness of your individual: {}", my_individual.fitness(
    ///     &DistanceMat::new(vec![vec![0.0,1.0,2.0], vec![1.0,0.0,3.0], vec![2.0,3.0,0.0]]))
    /// )
    /// ```
    ///
    fn fitness(&self, distance_mat: &DistanceMat) -> f64 {
        distance_mat.get_distance(&self.indexes[..])
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
