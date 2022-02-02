use crate::distance_mat::DistanceMat;
use crate::gen_traits::Individual;
use crate::subsequence::Subsequence;
use crate::utils::{change_order, get_random_elem_from_range, ordered_crossover, remove_elem};
use rand::seq::SliceRandom;
use std::cmp::max;

/// The `Route` is an invidiual in the traveling salemens problem that is a valid route.
#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub struct Route {
    /// The order in which the nodes should be visited.
    pub indexes: Vec<usize>,
}
impl Route {
    /// Create a new route based on a vector of indexes.
    ///
    /// # Arguments
    ///
    /// * `indexes` - The order in which the nodes are visited in the Traveling Salesman Problem.
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algo::route::Route;
    ///
    /// let my_individual = Route::from(Route::new(vec![0,1,2]));
    /// ```
    pub fn new(indexes: Vec<usize>) -> Self {
        Self { indexes }
    }
}
impl<'a> Individual<'a> for Route {
    // The Distance matrix is needed by the individuals to compute their fitness on.
    type IndividualCost = DistanceMat;
    /// Randomly changes the order of two nodes in the route
    ///
    /// # Arguments
    ///
    /// * `prob` - The probability with which the indexes will be changed
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algo::route::Route;
    /// use genetic_algo::gen_traits::Individual;
    ///
    /// let my_individual = Route::from(Route::new(vec![0,1,2]));
    /// let my_mutated_indiviual =  my_individual.mutate(1.0);
    /// ```
    fn mutate(self, prob: f32) -> Self {
        Route {
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
    /// * `other` - The other individual you would like to crossover with this individual.
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algo::route::Route;
    /// use genetic_algo::gen_traits::Individual;
    ///
    /// let my_individual = Route::from(Route::new(vec![0,1,2]));
    /// let my_individual = my_individual.crossover(
    ///     &Route::from(Route::new(vec![1,0,2]))
    /// );
    /// ```
    fn crossover(&self, other: &Route) -> Self {
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
    /// route
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algo::route::Route;
    /// use genetic_algo::distance_mat::DistanceMat;
    /// use genetic_algo::gen_traits::Individual;
    ///
    /// let my_individual = Route::from(Route::new(vec![0,1,2]));
    /// println!("Fitness of your individual: {}", my_individual.fitness(
    ///     &DistanceMat::new(vec![vec![0.0,1.0,2.0], vec![1.0,0.0,3.0], vec![2.0,3.0,0.0]]))
    /// )
    /// ```
    ///
    fn fitness(&self, distance_mat: &DistanceMat) -> f64 {
        -distance_mat.get_distance(&self.indexes[..])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod test_route {
        use super::*;
        use crate::test_utils::valid_permutation;
        #[test]
        fn test_constructor() {
            let route = Route::new(vec![1, 2, 3, 4]);
            assert_eq!(route.indexes, vec![1, 2, 3, 4])
        }
        #[test]
        fn test_mutuate_no_prob() {
            assert_eq!(
                Route::new(vec![1, 2, 3, 4]).mutate(0.0).indexes,
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
                Route::new(vec![1, 2, 3, 4]).mutate(1.0).indexes,
                vec![1, 2, 3, 4]
            )
        }
        #[test]
        fn test_mutuate_100_prob_3_elems() {
            assert_ne!(Route::new(vec![1, 2, 3]).mutate(1.0).indexes, vec![1, 2, 3])
        }
        #[test]
        fn test_mutate_simple_run() {
            let test_route = Route::new(vec![1, 2, 0]);
            valid_permutation(&test_route.indexes, &test_route.clone().mutate(0.5).indexes);
        }
    }
    mod test_crossover {
        use super::*;
        use crate::test_utils::valid_permutation;

        #[test]
        fn random_test_10() {
            let n_tests = 1000;
            let route_a = Route {
                indexes: vec![0, 12, 7, 3, 9, 8, 11, 5, 13, 1, 4, 6, 10, 15, 2, 14],
            };
            let route_b = Route {
                indexes: vec![7, 10, 15, 12, 2, 9, 5, 3, 1, 6, 4, 13, 14, 11, 8, 0],
            };
            let mut n_no_crossover = 0;
            for _ in 1..n_tests {
                let result = route_a.crossover(&route_b);
                if result.indexes == route_a.indexes || result.indexes == route_b.indexes {
                    n_no_crossover += 1;
                }
                valid_permutation(&result.indexes, &route_a.indexes);
                valid_permutation(&result.indexes, &route_a.indexes);
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
            let route = Route::new(vec![1, 2, 0]);
            assert_eq!(route.fitness(&distance_mat), -6.0);
        }
    }
}
