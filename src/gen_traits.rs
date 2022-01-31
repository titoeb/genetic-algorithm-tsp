use crate::utils::argsort;
use core::fmt::Debug;

/// Individual used in the genetic algorithm.
pub trait Individual<'a>: Debug + PartialEq + Clone + Eq {
    /// The Type of cost data this individual is compatible to compute its
    /// fitness on.
    type IndividualCost: 'a;
    /// Randomly changes the order of two nodes in the solution
    ///
    /// # Arguments
    ///
    /// * `prob` - The probability with which the indexes will be changed
    fn mutate(self, prob: f32) -> Self;
    /// Crossover this invidual with another individual to create a new individual. Currently
    /// uses the `ordered_crossover` algorithm.
    ///
    /// # Arguments
    ///
    /// * `other` - The other individual you would like to use in the crossover individual.
    ///
    fn crossover(&self, other: &Self) -> Self;
    /// Compute how much distance the individual implies with its order of nodes
    /// and the distance matrix.
    ///
    /// # Arguments
    ///
    /// * `distance_matrix` - Distance Matrix that determines the length of the proposed
    /// solution
    ///
    fn fitness(&self, cost_data: &Self::IndividualCost) -> f64;
    // {
    //     cost_data.compute_cost(self)
    // }
}

/// The container for your current solutions of your problem in a genetic algorithm.
pub trait Population<'a> {
    /// The types your the individuals in your genetic algorithm are that this population is
    /// compatible to.
    type Individual: Individual<'a> + 'a;
    /// The Iterator you return over your individuals. It depends on the data container you use
    /// to store individuals in your implementation of `Population`.
    type IndividualCollection: Iterator<Item = &'a <Self as Population<'a>>::Individual>;
    /// The type of data you use in your Population to generate the iterator.
    //type IndividualIterData;
    /// Given your pool of current solutions, compute the fitness of your individuals to solve the
    /// problem at hand.
    ///
    /// # Arguments
    ///
    /// * `distance_mat` - The distances between nodes that is neccessary to computes how well the solution
    /// work in terms of the TSP
    ///
    fn fitnesses(
        &'a self,
        cost_data: &'a <<Self as Population<'a>>::Individual as Individual<'a>>::IndividualCost,
    ) -> Vec<(f64, &Self::Individual)> {
        self.iter()
            .map(|solution| (solution.fitness(cost_data), solution))
            .collect()
    }
    /// Get the n fittest individuals in your routes.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of individuals you would like to have.
    /// * `cost_data` - The cost data structure your individuals need to compute
    /// their fitness.
    ///
    fn get_n_fittest(
        &'a self,
        n: usize,
        cost_data: &'a <<Self as Population<'a>>::Individual as Individual<'a>>::IndividualCost,
    ) -> Vec<Self::Individual> {
        let solutions_by_fitness = self.fitnesses(cost_data);
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
    /// * `cost_data` - The cost data structure your indivudals need to compute their fitness.
    ///
    fn get_fittest_population(
        &'a self,
        n: usize,
        cost_data: &'a <<Self as Population<'a>>::Individual as Individual<'a>>::IndividualCost,
    ) -> Self;
    /// Evolve your population.
    ///
    /// The evolution consists of the following stages:
    /// 1) `crossover` between all 1,...,n solutions excluding the solution itself.
    /// 2) `mutate` is applied to all individuals.
    ///
    /// # Arguments
    ///
    /// * `mutate_prob` - The probabilty of an inviduals beeing mutated. Is applied via `individuals.mutate`.
    fn evolve(&self, mutate_prob: f32) -> Self;
    /// TODO: DOCUMENTATION
    // TODO: I only use `Vec` here because the type of the iterator is too complicated.
    // this creates overhead and should be optimized
    fn evolve_individuals(&'a self, mutate_prob: f32) -> Vec<Self::Individual> {
        self
            // for all solutions 1 .. n crossover with all other solutions excluding the same solution.
            .iter()
            .enumerate()
            .map(|(idx, main_solution)| {
                self.iter()
                    // Skip the solution itself, e.g. don't crossover the solution with itself.
                    .enumerate()
                    .filter(move |&(solution_index, _)| solution_index != idx)
                    .map(|(_, solution)| main_solution.crossover(solution).mutate(mutate_prob))
            })
            .flatten()
            .chain(self.iter().cloned())
            .collect()
    }
    /// Iterate over the individuals in your population.
    fn iter(&'a self) -> Self::IndividualCollection;
}
