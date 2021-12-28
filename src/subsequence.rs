use crate::solution::Solution;
use crate::utils::get_elem_from_range;
use std::collections::HashSet;
#[derive(Debug)]
pub struct Subsequence {
    start_index: usize,
    length: usize,
}

impl Subsequence {
    pub fn new(start_index: usize, length: usize) -> Self {
        Subsequence {
            start_index,
            length,
        }
    }
    pub fn random_subsequence(max: usize) -> Self {
        let start_index = get_elem_from_range(0..(max - 1));
        Subsequence {
            start_index,
            length: get_elem_from_range((start_index + 1)..(max - start_index - 1)),
        }
    }
    pub fn get_unique_elems(&self, solution: &Solution) -> HashSet<usize> {
        solution.indexes[self.start_index..(self.start_index + self.length)]
            .iter()
            .copied()
            .collect()
    }
}
