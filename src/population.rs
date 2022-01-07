use crate::distance_mat::DistanceMat;
use crate::solution::Solution;
use crate::utils::{argsort, random_permutation};
use std::convert::From;

#[derive(Debug, Clone, PartialEq)]
pub struct Population {
    pub solutions: Vec<Solution>,
}

impl From<Vec<Solution>> for Population {
    fn from(solutions: Vec<Solution>) -> Self {
        Population { solutions }
    }
}

impl Population {
    pub fn random(n_individuals: usize, n_objects: usize) -> Self {
        let all_objects = (0..n_objects).collect::<Vec<usize>>();
        Population {
            solutions: (0..n_individuals)
                .map(|_| Solution::new(random_permutation(&all_objects)))
                .collect(),
        }
    }
    fn fitnesses(&self, distance_mat: &DistanceMat) -> Vec<f64> {
        self.solutions
            .iter()
            .map(|solution| solution.fitness(distance_mat))
            .collect()
    }
    pub fn get_n_fittest(&self, n: usize, distance_mat: &DistanceMat) -> Vec<Solution> {
        argsort(&self.fitnesses(distance_mat))
            .iter()
            .take(n)
            .map(|idx| self.solutions[*idx].clone())
            .collect()
    }
    pub fn get_fittest_population(&self, n: usize, distance_mat: &DistanceMat) -> Population {
        Population {
            solutions: self.get_n_fittest(n, distance_mat),
        }
    }
    pub fn evolve(&self, mutate_prob: f32) -> Population {
        Population {
            solutions: self
                .solutions
                .iter()
                .enumerate()
                .map(|(idx, main_solution)| {
                    self.solutions
                        .iter()
                        .skip(idx)
                        // .map(|solution| main_solution.crossover(solution))
                        // should look like:
                        .map(|solution| main_solution.crossover(solution).mutate(mutate_prob))
                })
                .flatten()
                .collect(),
        }
    }
}

pub fn evolve_population(
    initial_population: Population,
    n_generations: usize,
    size_generation: usize,
    distance_mat: &DistanceMat,
) -> Population {
    (0..n_generations).fold(initial_population, |pop, _| {
        pop.evolve(0.5)
            .get_fittest_population(size_generation, distance_mat)
    })
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
