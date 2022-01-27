use crate::distance_mat::DistanceMat;
use crate::gen_traits::{CostData, Individual, Population};

use crate::solution::Solution;
use crate::utils::{argsort, random_permutation};
use crossbeam_utils::thread;
use std::collections::HashSet;
use std::convert::From;
use std::time::Instant;

/// The `Population` is your current pools of solutions that you would to improve by evolving them.
#[derive(Debug, Clone, PartialEq)]
pub struct Routes {
    /// An individual routes is made from `solutions`, e.g. individuals that might your given problem
    /// better of worse.
    solutions: HashSet<Solution>,
}
// Convert a Vector of solutioons to a routes.
impl From<Vec<Solution>> for Routes {
    /// Create a new Population of Routse from a vector of solutions.
    ///
    /// # Arguments
    ///
    /// * `solutions` - The solutions you collected so far and would like to put into your
    /// routes.
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algo::routes::Routes;
    /// use genetic_algo::solution::Solution;
    ///
    /// let routes = Routes::from(vec![Solution::new(vec![0,1,2]), Solution::new(vec![1,0,2])]);
    /// ```
    fn from(solutions: Vec<Solution>) -> Self {
        Routes {
            solutions: solutions.into_iter().collect(),
        }
    }
}

impl Routes {
    /// Create a new Population of routes by creating random invidiual routes.
    ///
    /// # Arguments
    ///
    /// * `n_routse` - The number of routes your population of routes should contain.
    /// * `route_length` - The length of an individual route.
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algo::routes::Routes;
    /// use genetic_algo::solution::Solution;
    ///
    /// let routes = Routes::from(vec![Solution::new(vec![0,1,2]), Solution::new(vec![1,0,2])]);
    /// ```
    pub fn random(n_routes: usize, route_length: usize) -> Self {
        let all_objects = (0..route_length).collect::<Vec<usize>>();
        let mut solutions = HashSet::new();

        while solutions.len() < n_routes {
            solutions.insert(Solution::new(random_permutation(&all_objects)));
        }

        Routes { solutions }
    }
    fn iter(&self) -> impl Iterator<Item = &Solution> {
        self.solutions.iter()
    }
}

impl Population for Routes {
    type Individual = Solution;
    type CostData = DistanceMat;

    /// Given your pool of current solutions, compute the fitness of your individuals to solve the
    /// problem at hand.
    ///
    /// # Arguments
    ///
    /// * `distance_mat` - The distances between nodes that is neccessary to computes how well the solution
    /// work in terms of the TSP
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algo::routes::Routes;
    /// use genetic_algo::solution::Solution;
    /// use genetic_algo::distance_mat::DistanceMat;
    /// use crate::genetic_algo::gen_traits::Population;
    ///
    /// let distance_matrix = DistanceMat::new(vec![vec![0.0,1.0,2.0], vec![1.0,0.0,3.0], vec![2.0,3.0,0.0]]);
    /// let routes = Routes::from(vec![Solution::new(vec![0,1,2]), Solution::new(vec![1,0,2])]);
    /// println!("Your routes's fitnesses: {:?}", routes.fitnesses(&distance_matrix));
    /// ```
    fn fitnesses(&self, distance_mat: &DistanceMat) -> Vec<(f64, &Solution)> {
        self.iter()
            .map(|solution| (solution.fitness(distance_mat), solution))
            .collect()
    }
    /// Get the n fittest individuals in your routes.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of individuals you would like to have.
    /// * `distance_mat` - The distance matrix the fitness should be evaluated on.
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algo::routes::Routes;
    /// use genetic_algo::solution::Solution;
    /// use genetic_algo::distance_mat::DistanceMat;
    /// use crate::genetic_algo::gen_traits::Population;
    ///
    /// let distance_matrix = DistanceMat::new(vec![vec![0.0,1.0,2.0], vec![1.0,0.0,3.0], vec![2.0,3.0,0.0]]);
    /// let routes = Routes::from(vec![Solution::new(vec![0,1,2]), Solution::new(vec![1,0,2])]);
    /// println!("Your fittest individual: {:?}", routes.get_n_fittest(1, &distance_matrix));
    /// ```
    fn get_n_fittest(&self, n: usize, distance_mat: &DistanceMat) -> Vec<Solution> {
        //    ) -> Vec<Box<dyn Individual>> {
        let solutions_by_fitness = self.fitnesses(distance_mat);
        argsort(
            &solutions_by_fitness
                .iter()
                .map(|(fitness, _)| *fitness)
                .collect::<Vec<f64>>(),
        )
        .iter()
        .take(n)
        .map(|idx| solutions_by_fitness[*idx].1.clone())
        .collect()
    }

    /// Get the n fittest individuals in your routes as new routes object. This is typically used
    /// to select the top n inidividuals, before continuing to evolve the routes further.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of individuals you would like to have.
    /// * `distance_mat` - The distance matrix the fitness should be evaluated on.
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algo::routes::Routes;
    /// use genetic_algo::solution::Solution;
    /// use genetic_algo::distance_mat::DistanceMat;
    /// use crate::genetic_algo::gen_traits::Population;
    ///
    /// let distance_matrix = DistanceMat::new(vec![vec![0.0,1.0,2.0], vec![1.0,0.0,3.0], vec![2.0,3.0,0.0]]);
    /// let routes = Routes::from(vec![Solution::new(vec![0,1,2]), Solution::new(vec![1,0,2])]);
    /// let my_fittest_routes = routes.get_fittest_population(2, &distance_matrix);
    /// ```
    fn get_fittest_population(&self, n: usize, distance_mat: &DistanceMat) -> Routes {
        Routes::from(self.get_n_fittest(n, distance_mat))
    }
    /// Evolve your population.
    ///
    /// The evolution consists of the following stages:
    /// 1) `crossover` between all 1,...,n solutions excluding the solution itself.
    /// 2) `mutate` is applied to all individuals.
    ///
    /// # Arguments
    ///
    /// * `mutate_prob` - The probabilty of an inviduals beeing mutated. Is applied via `individuals.mutate`.
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algo::routes::Routes;
    /// use genetic_algo::solution::Solution;
    /// use crate::genetic_algo::gen_traits::Population;
    /// use genetic_algo::distance_mat::DistanceMat;
    ///
    /// let distance_matrix = DistanceMat::new(vec![vec![0.0,1.0,2.0], vec![1.0,0.0,3.0], vec![2.0,3.0,0.0]]);
    /// let routes = Routes::from(vec![Solution::new(vec![0,1,2]), Solution::new(vec![1,0,2])]);
    /// let evolved_routes = routes.evolve(0.5);
    /// ```
    fn evolve(&self, mutate_prob: f32) -> Routes {
        Routes {
            solutions: self
                // for all solutions 1 .. n crossover with all other solutions excluding the same solution.
                .iter()
                .enumerate()
                .map(|(idx, main_solution)| {
                    self.solutions
                        .iter()
                        // Skip the solution itself, e.g. don't crossover the solution with itself.
                        .enumerate()
                        .filter(move |&(solution_index, _)| solution_index != idx)
                        .map(|(_, solution)| main_solution.crossover(solution).mutate(mutate_prob))
                })
                .flatten()
                .chain(self.solutions.iter().cloned())
                .collect(),
        }
    }
}

/// Given an initial population evolve it for `n_generations` while keeping `size_generation`
/// individuals. The final population will be returned.
///
/// # Arguments
///
/// * `initial_population` - Your initial population that should be evolved.
/// * `n_generations` - How many times should your population be evolved?
/// * `size_generation` - How many individuals should be kept after evolving it.
/// * `distance_matrix` - The distance matrix on which the fitness will be computed on.
///
/// # Examples
///
/// ```
/// use genetic_algo::routes::{Routes, evolve_population};
/// use genetic_algo::solution::Solution;
/// use genetic_algo::distance_mat::DistanceMat;
///
/// let evolved_population = evolve_population(
///     Routes::from(vec![Solution::new(vec![0,1,2]), Solution::new(vec![1,0,2])]),
///     10,
///     10,
///     &DistanceMat::new(vec![vec![0.0,1.0,2.0], vec![1.0,0.0,3.0], vec![2.0,3.0,0.0]]),
///     0
/// );
/// ```
pub fn evolve_population(
    initial_population: Routes,
    n_generations: usize,
    size_generation: usize,
    distance_matrix: &DistanceMat,
    n_jobs: usize,
) -> Routes {
    if n_jobs == 0 {
        // single-thread
        (0..n_generations).fold(initial_population, |pop, _| {
            pop.evolve(0.5)
                .get_fittest_population(size_generation, distance_matrix)
        })
    } else {
        // Multi-threaded execution
        thread::scope(|s| {
            let mut result = Vec::new();
            for _ in 0..n_jobs {
                let this_population = initial_population.clone();
                result.push(s.spawn(move |_| -> Vec<Solution> {
                    (0..((n_generations / n_jobs) + 1))
                        .fold(this_population, |pop, _| {
                            pop.evolve(0.5)
                                .get_fittest_population(size_generation, distance_matrix)
                        })
                        .get_n_fittest(size_generation, distance_matrix)
                }))
            }
            Routes::from(
                result
                    .into_iter()
                    .map(|thread| thread.join().unwrap())
                    .flatten()
                    .collect::<Vec<Solution>>(),
            )
        })
        .unwrap()
    }
}
/// Compute the time in milliseconds that it takes for a genetic algorithm to run.
///
/// # Arguments
///
/// * `n_generations` - How many generations should the algorithm evolve?
/// * `size_generation` - How many individuals should be selected at the end of each
/// evolution step.
/// * `dist_mat` - What is the distance matrix for your TSP.
///
/// ```
pub fn benchmark_population(
    n_generations: usize,
    size_generation: usize,
    dist_mat: &DistanceMat,
    n_jobs: usize,
) -> (u64, f64) {
    // End-to-end test: does the error of the solution get down?
    let before = Instant::now();
    let final_population = evolve_population(
        Routes::random(size_generation, dist_mat.n_units()),
        n_generations,
        size_generation,
        dist_mat,
        n_jobs,
    );
    let duration = before.elapsed();
    let nanos = duration.subsec_nanos() as u64;
    (
        (1000 * 1000 * 1000 * duration.as_secs() + nanos) / (1000 * 1000),
        final_population.get_n_fittest(1, dist_mat)[0].fitness(dist_mat),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{test_dist_mat, valid_permutation};

    #[test]
    fn from_solutions_vector() {
        assert_eq!(
            Routes::from(vec![
                Solution {
                    indexes: vec![0, 1, 2]
                },
                Solution {
                    indexes: vec![0, 2, 1]
                }
            ])
            .solutions,
            HashSet::from([
                Solution {
                    indexes: vec![0, 1, 2]
                },
                Solution {
                    indexes: vec![0, 2, 1]
                }
            ])
        )
    }
    #[test]
    fn random_constructor() {
        let n_objects = 3;
        let population = Routes::random(3, n_objects);
        assert_eq!(population.solutions.len(), 3);
        for solution in population.solutions {
            valid_permutation(&solution.indexes, &(0..n_objects).collect::<Vec<usize>>());
        }
    }
    #[test]
    fn test_fitness() {
        let distance_mat = test_dist_mat();
        let population = Routes::from(vec![
            Solution::new(vec![1, 2, 0]),
            Solution::new(vec![1, 0]),
        ]);
        let fitnesses = population.fitnesses(&distance_mat);
        assert_eq!(fitnesses.len(), 2);

        for element in vec![
            (6.0, &Solution::new(vec![1, 2, 0])),
            (2.0, &Solution::new(vec![1, 0])),
        ] {
            assert!(fitnesses.contains(&element))
        }
    }
    mod test_get_n_fittest {
        use super::*;
        #[test]
        fn n_0_fittest() {
            let distance_mat = test_dist_mat();
            let routes = Routes::from(vec![
                Solution::new(vec![1, 2, 0]),
                Solution::new(vec![1, 0]),
                Solution::new(vec![2, 0]),
            ]);
            assert_eq!(routes.get_n_fittest(0, &distance_mat), vec![],)
        }
        #[test]
        fn n_1_fittest() {
            let distance_mat = test_dist_mat();
            let routes = Routes::from(vec![
                Solution::new(vec![1, 2, 0]),
                Solution::new(vec![1, 0]),
                Solution::new(vec![2, 0]),
            ]);
            assert_eq!(
                routes.get_n_fittest(1, &distance_mat),
                vec![Solution::new(vec![1, 0]),],
            )
        }
        #[test]
        fn n_2_fittest() {
            let distance_mat = test_dist_mat();
            let routes = Routes::from(vec![
                Solution::new(vec![1, 2, 0]),
                Solution::new(vec![1, 0]),
                Solution::new(vec![2, 0]),
            ]);
            assert_eq!(
                routes.get_n_fittest(2, &distance_mat),
                vec![Solution::new(vec![1, 0]), Solution::new(vec![2, 0]),],
            )
        }
        #[test]
        fn n_3_fittest() {
            let distance_mat = test_dist_mat();
            let routes = Routes::from(vec![
                Solution::new(vec![1, 2, 0]),
                Solution::new(vec![1, 0]),
                Solution::new(vec![2, 0]),
            ]);
            assert_eq!(
                routes.get_n_fittest(3, &distance_mat),
                vec![
                    Solution::new(vec![1, 0]),
                    Solution::new(vec![2, 0]),
                    Solution::new(vec![1, 2, 0]),
                ],
            )
        }
    }
    mod test_fittest_routes {
        use super::*;
        #[test]
        fn n_0_fittest() {
            let distance_mat = test_dist_mat();
            let routes = Routes::from(vec![
                Solution::new(vec![1, 2, 0]),
                Solution::new(vec![1, 0]),
                Solution::new(vec![2, 0]),
            ]);
            assert_eq!(
                routes.get_fittest_population(0, &distance_mat),
                Routes {
                    solutions: HashSet::new()
                },
            )
        }
        #[test]
        fn n_1_fittest() {
            let distance_mat = test_dist_mat();
            let routes = Routes::from(vec![
                Solution::new(vec![1, 2, 0]),
                Solution::new(vec![1, 0]),
                Solution::new(vec![2, 0]),
            ]);
            assert_eq!(
                routes.get_fittest_population(1, &distance_mat),
                Routes {
                    solutions: HashSet::from([Solution::new(vec![1, 0]),]),
                },
            )
        }
        #[test]
        fn n_2_fittest() {
            let distance_mat = test_dist_mat();
            let routes = Routes::from(vec![
                Solution::new(vec![1, 2, 0]),
                Solution::new(vec![1, 0]),
                Solution::new(vec![2, 0]),
            ]);
            assert_eq!(
                routes.get_fittest_population(2, &distance_mat),
                Routes {
                    solutions: HashSet::from([
                        Solution::new(vec![1, 0]),
                        Solution::new(vec![2, 0])
                    ]),
                },
            )
        }
        #[test]
        fn n_3_fittest() {
            let distance_mat = test_dist_mat();
            let routes = Routes::from(vec![
                Solution::new(vec![1, 2, 0]),
                Solution::new(vec![1, 0]),
                Solution::new(vec![2, 0]),
            ]);
            assert_eq!(
                routes.get_fittest_population(3, &distance_mat),
                Routes {
                    solutions: HashSet::from([
                        Solution::new(vec![1, 0]),
                        Solution::new(vec![2, 0]),
                        Solution::new(vec![1, 2, 0]),
                    ]),
                },
            )
        }
    }
    mod test_evolve {
        use super::*;
        use crate::test_utils::{test_dist_mat, valid_permutation};
        #[test]
        fn simple_test() {
            let distance_mat = test_dist_mat();
            let routes = Routes::from(vec![
                Solution::new(vec![1, 2, 0]),
                Solution::new(vec![1, 0, 2]),
                Solution::new(vec![2, 1, 0]),
            ]);

            // Test at least three members after evolving.
            // Test maximum fitness can never decrease.
            let past_max_fitness = routes.get_n_fittest(1, &distance_mat)[0].fitness(&distance_mat);
            let new_routes = routes.evolve(0.5).evolve(0.5);

            assert!(
                routes.get_n_fittest(1, &distance_mat)[0].fitness(&distance_mat)
                    <= past_max_fitness
            );
            assert!(new_routes.solutions.len() >= 3);
            for solution in new_routes.solutions {
                valid_permutation(&vec![0, 1, 2], &solution.indexes);
            }
        }
    }
}
