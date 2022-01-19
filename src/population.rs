use crate::distance_mat::DistanceMat;

use crate::solution::Solution;
use crate::utils::{argsort, random_permutation};
use std::convert::From;
use std::time::Instant;

/// The `Population` is your current pools of solutions that you would to improve by evolving them.
#[derive(Debug, Clone, PartialEq)]
pub struct Population {
    /// An individual population is made from `solutions`, e.g. individuals that might your given problem
    /// better of worse.
    pub solutions: Vec<Solution>,
}
// Convert a Vector of solutioons to a population.
impl From<Vec<Solution>> for Population {
    /// Create a new Population from a vector of solutions.
    ///
    /// # Arguments
    ///
    /// * `solutions` - The solutions you collected so far and would like to put into your
    /// population.
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algo::population::Population;
    /// use genetic_algo::solution::Solution;
    ///
    /// let my_population = Population::from(vec![Solution::new(vec![0,1,2]), Solution::new(vec![1,0,2])]);
    /// ```
    fn from(solutions: Vec<Solution>) -> Self {
        Population { solutions }
    }
}

impl Population {
    /// Create a new Population from a vector of solutions.
    ///
    /// # Arguments
    ///
    /// * `solutions` - The solutions you collected so far and would like to put into your
    /// population.
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algo::population::Population;
    /// use genetic_algo::solution::Solution;
    ///
    /// let my_population = Population::from(vec![Solution::new(vec![0,1,2]), Solution::new(vec![1,0,2])]);
    /// ```
    pub fn random(n_individuals: usize, n_objects: usize) -> Self {
        let all_objects = (0..n_objects).collect::<Vec<usize>>();
        Population {
            solutions: (0..n_individuals)
                .map(|_| Solution::new(random_permutation(&all_objects)))
                .collect(),
        }
    }
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
    /// use genetic_algo::population::Population;
    /// use genetic_algo::solution::Solution;
    /// use genetic_algo::distance_mat::DistanceMat;
    ///
    /// let distance_matrix = DistanceMat::new(vec![vec![0.0,1.0,2.0], vec![1.0,0.0,3.0], vec![2.0,3.0,0.0]]);
    /// let my_population = Population::from(vec![Solution::new(vec![0,1,2]), Solution::new(vec![1,0,2])]);
    /// println!("Your population's fitnesses: {:?}", my_population.fitnesses(&distance_matrix));
    /// ```
    pub fn fitnesses(&self, distance_mat: &DistanceMat) -> Vec<f64> {
        self.solutions
            .iter()
            .map(|solution| solution.fitness(distance_mat))
            .collect()
    }
    /// Get the n fittest individuals in your population.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of individuals you would like to have.
    /// * `distance_mat` - The distance matrix the fitness should be evaluated on.
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algo::population::Population;
    /// use genetic_algo::solution::Solution;
    /// use genetic_algo::distance_mat::DistanceMat;
    ///
    /// let distance_matrix = DistanceMat::new(vec![vec![0.0,1.0,2.0], vec![1.0,0.0,3.0], vec![2.0,3.0,0.0]]);
    /// let my_population = Population::from(vec![Solution::new(vec![0,1,2]), Solution::new(vec![1,0,2])]);
    /// println!("Your fittest individual: {:?}", my_population.get_n_fittest(1, &distance_matrix));
    /// ```
    pub fn get_n_fittest(&self, n: usize, distance_mat: &DistanceMat) -> Vec<Solution> {
        argsort(&self.fitnesses(distance_mat))
            .iter()
            .take(n)
            .map(|idx| self.solutions[*idx].clone())
            .collect()
    }

    /// Get the n fittest individuals in your population as new population object. This is typically used
    /// to select the top n inidividuals, before continuing to evolve the population further.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of individuals you would like to have.
    /// * `distance_mat` - The distance matrix the fitness should be evaluated on.
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algo::population::Population;
    /// use genetic_algo::solution::Solution;
    /// use genetic_algo::distance_mat::DistanceMat;
    ///
    /// let distance_matrix = DistanceMat::new(vec![vec![0.0,1.0,2.0], vec![1.0,0.0,3.0], vec![2.0,3.0,0.0]]);
    /// let my_population = Population::from(vec![Solution::new(vec![0,1,2]), Solution::new(vec![1,0,2])]);
    /// let my_fittest_population = my_population.get_fittest_population(2, &distance_matrix);
    /// ```
    pub fn get_fittest_population(&self, n: usize, distance_mat: &DistanceMat) -> Population {
        Population {
            solutions: self.get_n_fittest(n, distance_mat),
        }
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
    /// use genetic_algo::population::Population;
    /// use genetic_algo::solution::Solution;
    /// use genetic_algo::distance_mat::DistanceMat;
    ///
    /// let distance_matrix = DistanceMat::new(vec![vec![0.0,1.0,2.0], vec![1.0,0.0,3.0], vec![2.0,3.0,0.0]]);
    /// let my_population = Population::from(vec![Solution::new(vec![0,1,2]), Solution::new(vec![1,0,2])]);
    /// let evolved_population = my_population.evolve(0.5);
    /// ```
    pub fn evolve(&self, mutate_prob: f32) -> Population {
        Population {
            solutions: self
                // for all solutions 1 .. n crossover with all other solutions excluding the same solution.
                .solutions
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
/// use genetic_algo::population::{Population, evolve_population};
/// use genetic_algo::solution::Solution;
/// use genetic_algo::distance_mat::DistanceMat;
///
/// let evolved_population = evolve_population(
///     Population::from(vec![Solution::new(vec![0,1,2]), Solution::new(vec![1,0,2])]),
///     10,
///     10,
///     &DistanceMat::new(vec![vec![0.0,1.0,2.0], vec![1.0,0.0,3.0], vec![2.0,3.0,0.0]])
/// );
/// ```
pub fn evolve_population(
    initial_population: Population,
    n_generations: usize,
    size_generation: usize,
    distance_matrix: &DistanceMat,
) -> Population {
    (0..n_generations).fold(initial_population, |pop, _| {
        pop.evolve(0.5)
            .get_fittest_population(size_generation, distance_matrix)
    })
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
) -> (u64, f64) {
    // End-to-end test: does the error of the solution get down?
    let before = Instant::now();
    let final_population = evolve_population(
        Population::random(size_generation, dist_mat.n_units()),
        n_generations,
        size_generation,
        dist_mat,
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
            Population::from(vec![
                Solution {
                    indexes: vec![0, 1, 2]
                },
                Solution {
                    indexes: vec![0, 2, 1]
                }
            ])
            .solutions,
            vec![
                Solution {
                    indexes: vec![0, 1, 2]
                },
                Solution {
                    indexes: vec![0, 2, 1]
                }
            ]
        )
    }
    #[test]
    fn random_constructor() {
        let n_objects = 3;
        let population = Population::random(3, n_objects);
        assert_eq!(population.solutions.len(), 3);
        for solution in population.solutions {
            valid_permutation(&solution.indexes, &(0..n_objects).collect::<Vec<usize>>());
        }
    }
    #[test]
    fn test_fitness() {
        let distance_mat = test_dist_mat();
        let population = Population::from(vec![
            Solution::new(vec![1, 2, 0]),
            Solution::new(vec![1, 0]),
        ]);
        assert_eq!(population.fitnesses(&distance_mat), vec![6.0, 2.0],)
    }
    mod test_get_n_fittest {
        use super::*;
        #[test]
        fn n_0_fittest() {
            let distance_mat = test_dist_mat();
            let population = Population::from(vec![
                Solution::new(vec![1, 2, 0]),
                Solution::new(vec![1, 0]),
                Solution::new(vec![2, 0]),
            ]);
            assert_eq!(population.get_n_fittest(0, &distance_mat), vec![],)
        }
        #[test]
        fn n_1_fittest() {
            let distance_mat = test_dist_mat();
            let population = Population::from(vec![
                Solution::new(vec![1, 2, 0]),
                Solution::new(vec![1, 0]),
                Solution::new(vec![2, 0]),
            ]);
            assert_eq!(
                population.get_n_fittest(1, &distance_mat),
                vec![Solution::new(vec![1, 0]),],
            )
        }
        #[test]
        fn n_2_fittest() {
            let distance_mat = test_dist_mat();
            let population = Population::from(vec![
                Solution::new(vec![1, 2, 0]),
                Solution::new(vec![1, 0]),
                Solution::new(vec![2, 0]),
            ]);
            assert_eq!(
                population.get_n_fittest(2, &distance_mat),
                vec![Solution::new(vec![1, 0]), Solution::new(vec![2, 0]),],
            )
        }
        #[test]
        fn n_3_fittest() {
            let distance_mat = test_dist_mat();
            let population = Population::from(vec![
                Solution::new(vec![1, 2, 0]),
                Solution::new(vec![1, 0]),
                Solution::new(vec![2, 0]),
            ]);
            assert_eq!(
                population.get_n_fittest(3, &distance_mat),
                vec![
                    Solution::new(vec![1, 0]),
                    Solution::new(vec![2, 0]),
                    Solution::new(vec![1, 2, 0]),
                ],
            )
        }
    }
    mod test_fittest_population {
        use super::*;
        #[test]
        fn n_0_fittest() {
            let distance_mat = test_dist_mat();
            let population = Population::from(vec![
                Solution::new(vec![1, 2, 0]),
                Solution::new(vec![1, 0]),
                Solution::new(vec![2, 0]),
            ]);
            assert_eq!(
                population.get_fittest_population(0, &distance_mat),
                Population { solutions: vec![] },
            )
        }
        #[test]
        fn n_1_fittest() {
            let distance_mat = test_dist_mat();
            let population = Population::from(vec![
                Solution::new(vec![1, 2, 0]),
                Solution::new(vec![1, 0]),
                Solution::new(vec![2, 0]),
            ]);
            assert_eq!(
                population.get_fittest_population(1, &distance_mat),
                Population {
                    solutions: vec![Solution::new(vec![1, 0]),],
                },
            )
        }
        #[test]
        fn n_2_fittest() {
            let distance_mat = test_dist_mat();
            let population = Population::from(vec![
                Solution::new(vec![1, 2, 0]),
                Solution::new(vec![1, 0]),
                Solution::new(vec![2, 0]),
            ]);
            assert_eq!(
                population.get_fittest_population(2, &distance_mat),
                Population {
                    solutions: vec![Solution::new(vec![1, 0]), Solution::new(vec![2, 0])],
                },
            )
        }
        #[test]
        fn n_3_fittest() {
            let distance_mat = test_dist_mat();
            let population = Population::from(vec![
                Solution::new(vec![1, 2, 0]),
                Solution::new(vec![1, 0]),
                Solution::new(vec![2, 0]),
            ]);
            assert_eq!(
                population.get_fittest_population(3, &distance_mat),
                Population {
                    solutions: vec![
                        Solution::new(vec![1, 0]),
                        Solution::new(vec![2, 0]),
                        Solution::new(vec![1, 2, 0]),
                    ],
                },
            )
        }
    }
    mod test_evolve {
        use super::*;
        use crate::test_utils::valid_permutation;
        #[test]
        fn simple_test() {
            let distance_mat = test_dist_mat();
            let population = Population::from(vec![
                Solution::new(vec![1, 2, 0]),
                Solution::new(vec![1, 0, 2]),
                Solution::new(vec![2, 1, 0]),
            ]);
            let new_population = population.evolve(0.5);

            assert_eq!(new_population.solutions.len(), 6);
            for solution in new_population.solutions {
                valid_permutation(&vec![0, 1, 2], &solution.indexes);
            }
        }
    }
}
