use crate::solution::Solution;
use core::fmt::Debug;

/// Data used for the computation of the costs in genetic algorithms.
pub trait CostData {
    /// The individual that can deal with this cost data.
    type Individual;
    /// Compute the costs (reverse fitness) of an individual.
    ///
    /// # Arguments
    ///
    /// * `solution` - The individual that should be tested.
    ///
    fn compute_cost(&self, individual: &Self::Individual) -> f64;
}

/// Individual used in the genetic algorithm.
pub trait Individual: Debug + PartialEq + Clone + Eq {
    /// The Type of cost data this individual is compatible to compute its
    /// fitness on.
    type IndividualCost;
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
pub trait Population {
    /// The types your the individuals in your genetic algorithm are that this population is
    /// compatible to.
    type Individual;
    /// The object you store additional data in to compute the fitness of individuals. Your
    /// individuals as well as population need to be compatible in your implementation.
    type CostData;
    /// Given your pool of current solutions, compute the fitness of your individuals to solve the
    /// problem at hand.
    ///
    /// # Arguments
    ///
    /// * `distance_mat` - The distances between nodes that is neccessary to computes how well the solution
    /// work in terms of the TSP
    ///
    fn fitnesses(&self, cost_data: &Self::CostData) -> Vec<(f64, &Self::Individual)>;
    /// Get the n fittest individuals in your routes.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of individuals you would like to have.
    /// * `cost_data` - The cost data structure your individuals need to compute
    /// their fitness.
    ///
    fn get_n_fittest(&self, n: usize, cost_data: &Self::CostData) -> Vec<Self::Individual>;
    /// Get the n fittest individuals in your routes as new routes object. This is typically used
    /// to select the top n inidividuals, before continuing to evolve the routes further.
    ///
    /// # Arguments
    ///
    /// * `n` - The number of individuals you would like to have.
    /// * `cost_data` - The cost data structure your indivudals need to compute their fitness.
    ///
    fn get_fittest_population(&self, n: usize, cost_data: &Self::CostData) -> Self;
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
}
