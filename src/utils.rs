use crate::solution::Solution;
use crate::subsequence::Subsequence;
use rand::Rng;
use std::cmp::max;
use std::ops::Range;

pub fn get_elem_from_range<T>(range: Range<T>) -> T
where
    T: std::cmp::PartialOrd + rand::distributions::uniform::SampleUniform,
{
    rand::thread_rng().gen_range::<T, Range<T>>(range)
}
pub fn change_order(data: &[usize], put_before_idx: usize, move_idx: usize) -> Vec<usize> {
    let mut new_data = data.to_owned();
    let move_item = data[move_idx];
    new_data.remove(move_idx);
    new_data.insert(max(put_before_idx, 1) - 1, move_item);
    new_data
}

pub fn remove_elem(mut data: Vec<usize>, elem_idx: &usize) -> Vec<usize> {
    data.remove(*elem_idx);
    data
}

pub fn ordered_crossover(
    parent_a: Solution,
    parent_b: Solution,
    subsequence: Subsequence,
) -> Solution {
    unimplemented!();
}
