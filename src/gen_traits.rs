use crate::utils::argsort;
use core::fmt::Debug;

/// A single instance in the genetic algorithm.
/// In a TSP for example, this would be and individual route.
pub trait Individual<'a>: Debug + PartialEq + Eq + Clone {
    /// The Type of cost data this individual is compatible to compute its
    /// fitness on.
    type IndividualCost: 'a;
    /// Randomly change the object and therefore it's fitness
    /// This is a key step of the genetic algorithm.
    ///
    /// # Arguments
    ///
    /// * `prob` - The probability with which the individual will mutate.
    fn mutate(self, prob: f32) -> Self;
    /// The `crossover` takes two individual and combines their characteristics.
    /// The implementation depends on the problem to be solve with genetic algorithm but
    /// both from a performance and runtime perspective, this is one of the most important
    /// and time-consuming methods.
    ///
    /// # Arguments
    ///
    /// * `other` - The other individual that should be `crossover`ed with.
    ///
    fn crossover(&self, other: &Self) -> Self;
    /// How `fit` is your individual, e.g. how well does it solve the problem you are
    /// trying to solve with genetic algorithms. This is the metric that is maximised, e.g.
    /// overall individuals with a very high fitness should be found.
    ///
    /// # Arguments
    ///
    /// * `cost_data` - The data that might be needed to compute your fitness. If you use
    /// genetic algorithm to solve a traveling salesman problem, the `cost_data` will typically
    /// contain your distance matrix.
    ///
    fn fitness(&self, cost_data: &Self::IndividualCost) -> f64;
}

/// The container for your individuals in a genetic algorithm.
pub trait Population<'a> {
    /// The Type of individuals your population should consist of.
    type Individual: Individual<'a> + 'a;
    /// The iteratore type if you iterate over your individuals. It depends on the data container you use
    /// to store individuals in your implementation of `Population`.
    type IndividualCollection: Iterator<Item = &'a <Self as Population<'a>>::Individual>;
    /// Given the pool of current individuals, compute the fitness of your individuals to solve the
    /// problem at hand.
    ///
    /// # Arguments
    ///
    /// * `cost_data` - The data neccessary to assess the fitness of an individual.
    ///
    fn fitnesses(
        &'a self,
        cost_data: &'a <<Self as Population<'a>>::Individual as Individual<'a>>::IndividualCost,
    ) -> Vec<(f64, &Self::Individual)> {
        self.iter()
            .map(|solution| (solution.fitness(cost_data), solution))
            .collect()
    }
    /// Get the n fittest individuals in your population.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of individuals you would like to get
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
    /// Get the n fittest individuals in your population as population routes object. This is typically used
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
    /// 1) `crossover` between all 1,...,n indivials. Each individual will not be `crossover`ed
    /// with itself, but as the crossover is not neccessarily symmetric `indivdual_a.crossover(indivual_b)` as well
    /// as `individual_b.crossover(individual_a)` will be computed.
    /// 2) `mutate` is applied to all individuals.
    ///
    /// The difference to the `evolve_individuals` function is that this needs to be implemented in the struct
    /// that implements this trait because how the population is constructed depends on the representation you
    /// choose to use. Please use the `evolve_individuals`-function to get the updated inviduals. You could use an
    /// iterator or implement the `From<Vec<Individuals>>`-trait.
    ///
    /// # Arguments
    ///
    /// * `mutate_prob` - The probabilty of an inviduals beeing mutated. Is applied via `individuals.mutate`.
    fn evolve(&self, mutate_prob: f32) -> Self;
    /// Evolve your population.
    ///
    /// The evolution consists of the following stages:
    /// 1) `crossover` between all 1,...,n indivials. Each individual will not be `crossover`ed
    /// with itself, but as the crossover is not neccessarily symmetric `indivdual_a.crossover(indivual_b)` as well
    /// as `individual_b.crossover(individual_a)` will be computed.
    /// 2) `mutate` is applied to all individuals.
    ///
    /// # Arguments
    ///
    /// * `mutate_prob` - The probabilty of an inviduals beeing mutated. Is applied via `individuals.mutate`.
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
