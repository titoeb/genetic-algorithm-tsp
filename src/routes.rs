use crate::distance_mat::DistanceMat;
use crate::route::Route;
use crate::utils::random_permutation;
use crossbeam_utils::thread;
use fasthash_fork::xx;
use genetic_algorithm_traits::{Individual, Population};
use std::collections::HashSet;
use std::convert::From;
use std::fmt;
use std::time::Instant;

/// From a vector of routes create a Hashet with capacity length and hash function `xx-hash`.
///
/// # Arguments
///
/// * `routes` - The routes that should be added to the hashset.
///
fn route_vec_to_xx_hashset(routes: Vec<Route>) -> HashSet<Route, xx::Hash64> {
    let n_routes = routes.len();
    let mut routes_as_hashset = HashSet::with_capacity_and_hasher(n_routes, xx::Hash64);
    for route in routes {
        routes_as_hashset.insert(route);
    }
    routes_as_hashset
}

/// The `Population` is your current pools of routes that you would to improve by evolving them.
#[derive(Debug, Clone, PartialEq)]
pub struct Routes {
    /// An individual routes is made from `routes`, e.g. individuals that might your given problem
    /// better of worse.
    routes: HashSet<Route, xx::Hash64>,
}
impl fmt::Display for Routes {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "Routes([{}\n])",
            self.iter()
                .map(|route| format!("{}", route))
                .collect::<Vec<String>>()
                .join("\n\t")
        )
    }
}

// Convert a Vector of solutioons to a routes.
impl From<Vec<Route>> for Routes {
    /// Create a new Population of Routse from a vector of routes.
    ///
    /// # Arguments
    ///
    /// * `routes` - The routes you collected so far and would like to put into your
    /// routes.
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algorithm_tsp::routes::Routes;
    /// use genetic_algorithm_tsp::route::Route;
    ///
    /// let routes = Routes::from(vec![Route::new(vec![0,1,2]), Route::new(vec![1,0,2])]);
    /// ```
    fn from(routes: Vec<Route>) -> Self {
        // When this this will be `evolved` at most n_routes * (n_routes - 1) new
        // routes will be generate and all `n_routes` will be retained.
        Routes {
            routes: route_vec_to_xx_hashset(routes),
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
    /// use genetic_algorithm_tsp::routes::Routes;
    /// use genetic_algorithm_tsp::route::Route;
    ///
    /// let routes = Routes::from(vec![Route::new(vec![0,1,2]), Route::new(vec![1,0,2])]);
    /// ```
    pub fn random(n_routes: usize, route_length: usize) -> Self {
        let all_objects = (0..route_length).collect::<Vec<usize>>();
        let mut routes = HashSet::with_capacity_and_hasher(n_routes, xx::Hash64);

        while routes.len() < n_routes {
            routes.insert(Route::new(random_permutation(&all_objects)));
        }

        Routes { routes }
    }
    /// Add new routes to a `Routes`-object and create a new `Routes`-object
    ///
    /// # Arguments
    ///
    /// * `routes` - A vector of `Route`s that should be added.
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algorithm_tsp::routes::Routes;
    /// use genetic_algorithm_tsp::route::Route;
    ///
    /// let current_routes = Routes::from(vec![Route::new(vec![1]), Route::new(vec![2])]);
    /// let extended_routes = current_routes.add_vec_route(vec![Route::new(vec![3]), Route::new(vec![4])]);
    ///
    /// ```
    pub fn add_vec_route(self, routes: Vec<Route>) -> Self {
        Routes::from(
            self.routes
                .iter()
                .chain(routes.iter())
                .cloned()
                .collect::<Vec<Route>>(),
        )
    }
    /// Combine two routes objects.
    ///
    /// # Arguments
    ///
    /// * `routes` - A vector of `Route`s that should be added.
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algorithm_tsp::routes::Routes;
    /// use genetic_algorithm_tsp::route::Route;
    ///
    /// let current_routes = Routes::from(vec![Route::new(vec![1]), Route::new(vec![2])]);
    /// let other_routes = Routes::from(vec![Route::new(vec![3]), Route::new(vec![4])]);
    /// println!("{}", current_routes.combine_routes(other_routes));
    /// ```
    pub fn combine_routes(self, other_routes: Routes) -> Self {
        self.add_vec_route(other_routes.iter().cloned().collect::<Vec<Route>>())
    }
    /// Get the number of nodes for the `Route`'s in this `Routes`-object.
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algorithm_tsp::route::Route;
    /// use genetic_algorithm_tsp::routes::Routes;
    ///
    /// let routes_with_three_nodes = Routes::from(vec![Route::new(vec![1,2,3,]), Route::new(vec![4,5,6])]);
    /// println!("The route have {} nodes", routes_with_three_nodes.get_n_nodes());
    /// ```
    pub fn get_n_nodes(&self) -> usize {
        *self
            .iter()
            .take(1)
            .map(|node| node.get_n_nodes())
            .collect::<Vec<usize>>()
            .first()
            .unwrap()
    }
    /// Add n random nodes to your current pool.
    ///
    /// # Arguments:
    ///
    /// `n_random_nodes`: The number of random nodes that should be added.
    ///
    /// # Examples
    /// ```
    /// use genetic_algorithm_tsp::route::Route;
    /// use genetic_algorithm_tsp::routes::Routes;
    ///
    /// let a_single_route = Routes::from(vec![Route::new(vec![0,1,2])]);
    /// println!("{}", a_single_route.add_n_random_nodes(1));
    /// ```
    pub fn add_n_random_nodes(self, n_random_nodes: usize) -> Self {
        let number_of_nodes = self.get_n_nodes();
        self.combine_routes(Routes::random(n_random_nodes, number_of_nodes))
    }
}

impl<'a> Population<'a> for Routes {
    type Individual = Route;
    type IndividualCollection = std::collections::hash_set::Iter<'a, Route>;

    /// Given your pool of current routes, compute the fitness of your individuals to solve the
    /// problem at hand.
    ///
    /// # Arguments
    ///
    /// * `distance_mat` - The distances between nodes that is neccessary to computes how well the route
    /// work in terms of the TSP
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algorithm_tsp::routes::Routes;
    /// use genetic_algorithm_tsp::route::Route;
    /// use genetic_algorithm_tsp::distance_mat::DistanceMat;
    /// use genetic_algorithm_traits::Population;
    ///
    /// let distance_matrix = DistanceMat::new(vec![vec![0.0,1.0,2.0], vec![1.0,0.0,3.0], vec![2.0,3.0,0.0]]);
    /// let routes = Routes::from(vec![Route::new(vec![0,1,2]), Route::new(vec![1,0,2])]);
    /// println!("Your routes's fitnesses: {:?}", routes.fitnesses(&distance_matrix));
    /// ```
    // fn fitnesses(&self, distance_mat: &DistanceMat) -> Vec<(f64, &Route)> {
    //     self.iter()
    //         .map(|route| (route.fitness(distance_mat), route))
    //         .collect()
    // }
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
    /// use genetic_algorithm_tsp::routes::Routes;
    /// use genetic_algorithm_tsp::route::Route;
    /// use genetic_algorithm_tsp::distance_mat::DistanceMat;
    /// use genetic_algorithm_traits::Population;
    ///
    /// let distance_matrix = DistanceMat::new(vec![vec![0.0,1.0,2.0], vec![1.0,0.0,3.0], vec![2.0,3.0,0.0]]);
    /// let routes = Routes::from(vec![Route::new(vec![0,1,2]), Route::new(vec![1,0,2])]);
    /// let my_fittest_routes = routes.get_fittest_population(2, &distance_matrix);
    /// ```
    fn get_fittest_population(&self, n: usize, distance_mat: &DistanceMat) -> Routes {
        Routes {
            routes: route_vec_to_xx_hashset(self.get_n_fittest(n, distance_mat)),
        }
    }
    /// Evolve your population.
    ///
    /// The evolution consists of the following stages:
    /// 1) `crossover` between all 1,...,n routes excluding the route itself.
    /// 2) `mutate` is applied to all individuals.
    ///
    /// # Arguments
    ///
    /// * `mutate_prob` - The probabilty of an inviduals beeing mutated. Is applied via `individuals.mutate`.
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algorithm_tsp::routes::Routes;
    /// use genetic_algorithm_tsp::route::Route;
    /// use genetic_algorithm_traits::Population;
    /// use genetic_algorithm_tsp::distance_mat::DistanceMat;
    ///
    /// let distance_matrix = DistanceMat::new(vec![vec![0.0,1.0,2.0], vec![1.0,0.0,3.0], vec![2.0,3.0,0.0]]);
    /// let routes = Routes::from(vec![Route::new(vec![0,1,2]), Route::new(vec![1,0,2])]);
    /// let evolved_routes = routes.evolve(0.5);
    /// ```
    fn evolve(&self, mutate_prob: f32) -> Routes {
        let mutated_individuals = self.evolve_individuals(mutate_prob);
        Routes {
            routes: route_vec_to_xx_hashset(mutated_individuals),
        }
    }
    /// Iterate over the individuals of your population.
    ///
    /// # Examples
    ///
    /// ```
    /// use genetic_algorithm_tsp::routes::Routes;
    /// use genetic_algorithm_tsp::route::Route;
    /// use genetic_algorithm_traits::Population;
    ///
    /// let routes = Routes::from(vec![Route::new(vec![0,1,2]), Route::new(vec![1,0,2])]);
    /// for route in routes.iter(){
    ///     println!("{:?}", route);
    /// }
    /// ```
    fn iter(&'a self) -> std::collections::hash_set::Iter<Route> {
        self.routes.iter()
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
/// use genetic_algorithm_tsp::routes::{Routes, evolve_population};
/// use genetic_algorithm_tsp::route::Route;
/// use genetic_algorithm_tsp::distance_mat::DistanceMat;
///
/// let evolved_population = evolve_population(
///     Routes::from(vec![Route::new(vec![0,1,2]), Route::new(vec![1,0,2])]),
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
                result.push(s.spawn(move |_| -> Vec<Route> {
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
                    .flat_map(|thread| thread.join().unwrap())
                    .collect::<Vec<Route>>(),
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
    // End-to-end test: does the error of the route get down?
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
    fn test_route_vec_to_xx_hashset() {
        let routes_vec = vec![
            Route::new(vec![0, 1, 2]),
            Route::new(vec![0, 1, 2]),
            Route::new(vec![1, 0, 2]),
        ];
        let routes_as_hashet: HashSet<Route, xx::Hash64> =
            route_vec_to_xx_hashset(routes_vec.clone());
        // Routes in the hashset are unique, so the duplicate in `routes_vec`
        // should only be in there once.
        assert_eq!(routes_as_hashet.len(), 2);
        // But all routes from route_vec should be in there.
        for route in &routes_vec {
            assert!(routes_as_hashet.contains(route))
        }
    }
    #[test]
    fn test_format() {
        let route_to_print = Routes::from(vec![Route::new(vec![1, 2])]);
        assert_eq!(format!("{}", route_to_print), "Routes([Route([1, 2])\n])");
    }
    #[test]
    fn from_routes_vector() {
        assert_eq!(
            Routes::from(vec![
                Route {
                    indexes: vec![0, 1, 2]
                },
                Route {
                    indexes: vec![0, 2, 1]
                }
            ])
            .routes,
            route_vec_to_xx_hashset(vec![
                Route {
                    indexes: vec![0, 1, 2]
                },
                Route {
                    indexes: vec![0, 2, 1]
                }
            ],)
        )
    }

    #[test]
    fn random_constructor() {
        let n_objects = 3;
        let population = Routes::random(3, n_objects);
        assert_eq!(population.routes.len(), 3);
        for route in population.routes {
            valid_permutation(&route.indexes, &(0..n_objects).collect::<Vec<usize>>());
        }
    }
    #[test]
    fn test_add_vec_routes() {
        let current_routes = Routes::from(vec![Route::new(vec![1]), Route::new(vec![2])]);
        let extended_routes =
            current_routes.add_vec_route(vec![Route::new(vec![3]), Route::new(vec![4])]);

        valid_permutation(
            &vec![
                Route::new(vec![1]),
                Route::new(vec![2]),
                Route::new(vec![3]),
                Route::new(vec![4]),
            ],
            &extended_routes
                .iter()
                .map(|route| route.clone())
                .collect::<Vec<Route>>(),
        )
    }
    #[test]
    fn test_combine_routes() {
        let current_routes = Routes::from(vec![Route::new(vec![1]), Route::new(vec![2])]);
        let other_routes = Routes::from(vec![Route::new(vec![3]), Route::new(vec![4])]);
        let combined_routes = current_routes.combine_routes(other_routes);
        valid_permutation(
            &vec![
                Route::new(vec![1]),
                Route::new(vec![2]),
                Route::new(vec![3]),
                Route::new(vec![4]),
            ],
            &combined_routes
                .iter()
                .map(|route| route.clone())
                .collect::<Vec<Route>>(),
        )
    }
    #[test]
    fn test_get_n_nodes() {
        let routes_with_three_nodes =
            Routes::from(vec![Route::new(vec![1, 2, 3]), Route::new(vec![4, 5, 6])]);
        assert_eq!(routes_with_three_nodes.get_n_nodes(), 3);
    }
    #[test]
    fn add_n_random_nodes() {
        // Because there are only 6 possible routes with three nodes,
        // when I add 6, there have to be 6 in total (e.g. five new ones
        // were added).
        let a_single_route = Routes::from(vec![Route::new(vec![0, 1, 2])]);
        assert_eq!(a_single_route.add_n_random_nodes(6).iter().len(), 6);
    }
    #[test]
    fn test_fitness() {
        let distance_mat = test_dist_mat();
        let population = Routes::from(vec![Route::new(vec![1, 2, 0]), Route::new(vec![1, 0])]);
        let fitnesses = population.fitnesses(&distance_mat);
        assert_eq!(fitnesses.len(), 2);

        for element in vec![
            (-6.0, &Route::new(vec![1, 2, 0])),
            (-2.0, &Route::new(vec![1, 0])),
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
                Route::new(vec![1, 2, 0]),
                Route::new(vec![1, 0]),
                Route::new(vec![2, 0]),
            ]);
            assert_eq!(routes.get_n_fittest(0, &distance_mat), vec![],)
        }
        #[test]
        fn n_1_fittest() {
            let distance_mat = test_dist_mat();
            let routes = Routes::from(vec![
                Route::new(vec![1, 2, 0]),
                Route::new(vec![1, 0]),
                Route::new(vec![2, 0]),
            ]);
            assert_eq!(
                routes.get_n_fittest(1, &distance_mat),
                vec![Route::new(vec![1, 0]),],
            )
        }
        #[test]
        fn n_2_fittest() {
            let distance_mat = test_dist_mat();
            let routes = Routes::from(vec![
                Route::new(vec![1, 2, 0]),
                Route::new(vec![1, 0]),
                Route::new(vec![2, 0]),
            ]);
            assert_eq!(
                routes.get_n_fittest(2, &distance_mat),
                vec![Route::new(vec![1, 0]), Route::new(vec![2, 0]),],
            )
        }
        #[test]
        fn n_3_fittest() {
            let distance_mat = test_dist_mat();
            let routes = Routes::from(vec![
                Route::new(vec![1, 2, 0]),
                Route::new(vec![1, 0]),
                Route::new(vec![2, 0]),
            ]);
            assert_eq!(
                routes.get_n_fittest(3, &distance_mat),
                vec![
                    Route::new(vec![1, 0]),
                    Route::new(vec![2, 0]),
                    Route::new(vec![1, 2, 0]),
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
                Route::new(vec![1, 2, 0]),
                Route::new(vec![1, 0]),
                Route::new(vec![2, 0]),
            ]);
            assert_eq!(
                routes.get_fittest_population(0, &distance_mat),
                Routes {
                    routes: HashSet::with_hasher(xx::Hash64),
                },
            )
        }
        #[test]
        fn n_1_fittest() {
            let distance_mat = test_dist_mat();
            let routes = Routes::from(vec![
                Route::new(vec![1, 2, 0]),
                Route::new(vec![1, 0]),
                Route::new(vec![2, 0]),
            ]);
            assert_eq!(
                routes.get_fittest_population(1, &distance_mat),
                Routes {
                    routes: route_vec_to_xx_hashset(vec![Route::new(vec![1, 0]),],),
                },
            )
        }
        #[test]
        fn n_2_fittest() {
            let distance_mat = test_dist_mat();
            let routes = Routes::from(vec![
                Route::new(vec![1, 2, 0]),
                Route::new(vec![1, 0]),
                Route::new(vec![2, 0]),
            ]);
            assert_eq!(
                routes.get_fittest_population(2, &distance_mat),
                Routes {
                    routes: route_vec_to_xx_hashset(vec![
                        Route::new(vec![1, 0]),
                        Route::new(vec![2, 0])
                    ],),
                },
            )
        }
        #[test]
        fn n_3_fittest() {
            let distance_mat = test_dist_mat();
            let routes = Routes::from(vec![
                Route::new(vec![1, 2, 0]),
                Route::new(vec![1, 0]),
                Route::new(vec![2, 0]),
            ]);
            assert_eq!(
                routes.get_fittest_population(3, &distance_mat),
                Routes {
                    routes: route_vec_to_xx_hashset(vec![
                        Route::new(vec![1, 0]),
                        Route::new(vec![2, 0]),
                        Route::new(vec![1, 2, 0]),
                    ],),
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
                Route::new(vec![1, 2, 0]),
                Route::new(vec![1, 0, 2]),
                Route::new(vec![2, 1, 0]),
            ]);

            // Test at least three members after evolving.
            // Test maximum fitness can never decrease.
            let past_max_fitness = routes.get_n_fittest(1, &distance_mat)[0].fitness(&distance_mat);
            let new_routes = routes.evolve(0.5).evolve(0.5);

            assert!(
                routes.get_n_fittest(1, &distance_mat)[0].fitness(&distance_mat)
                    <= past_max_fitness
            );
            assert!(new_routes.routes.len() >= 3);
            for route in new_routes.routes {
                valid_permutation(&vec![0, 1, 2], &route.indexes);
            }
        }
    }
    #[test]
    fn test() {
        let mut set = HashSet::with_capacity_and_hasher(1000, xx::Hash64);
        set.insert(Route::new(vec![1, 2, 3]));
    }
}
