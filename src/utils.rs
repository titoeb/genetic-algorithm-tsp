use crate::solution::Solution;
use crate::subsequence::Subsequence;
use rand::Rng;
use std::cmp::max;
use std::ops::Range;

pub fn get_elem_from_range<T>(range: Range<T>) -> T
where
    T: std::cmp::PartialOrd + rand::distributions::uniform::SampleUniform,
{
    if !range.is_empty() {
        rand::thread_rng().gen_range::<T, Range<T>>(range)
    } else {
        range.start
    }
}
pub fn change_order(data: &[usize], put_before_idx: usize, move_idx: usize) -> Vec<usize> {
    let mut new_data = data.to_owned();
    if put_before_idx != move_idx {
        let move_item = data[move_idx];
        new_data.remove(move_idx);
        let reset_index = (move_idx < put_before_idx) as usize;
        new_data.insert(
            max(put_before_idx, reset_index as usize) - reset_index as usize,
            move_item,
        );
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    mod get_elem_from_range {
        use super::*;
        #[test]
        fn sample_int_range() {
            get_elem_from_range(0..10);
        }
        #[test]
        fn sample_float_range() {
            get_elem_from_range(0.0..1.0);
        }
        #[test]
        fn sample_empty_range() {
            assert_eq!(get_elem_from_range(0..0), 0);
        }
    }
    mod test_remove_elem {
        use super::*;
        #[test]
        fn remove_first() {
            assert_eq!(remove_elem(vec![1, 2, 3, 4], &0), vec![2, 3, 4]);
        }
        #[test]
        fn remove_last() {
            assert_eq!(remove_elem(vec![1, 2, 3, 4], &3), vec![1, 2, 3]);
        }
        #[test]
        fn remove_middle() {
            assert_eq!(remove_elem(vec![1, 2, 3, 4], &2), vec![1, 2, 4]);
        }
        #[test]
        fn test_remove_elem_first() {
            assert_eq!(remove_elem(vec![1, 2, 3], &0), vec![2, 3])
        }
        #[test]
        fn test_remove_elem_middle() {
            assert_eq!(remove_elem(vec![1, 2, 3], &1), vec![1, 3])
        }
        #[test]
        fn test_remove_elem_last() {
            assert_eq!(remove_elem(vec![1, 2, 3], &2), vec![1, 2])
        }
    }
    mod test_change_elem {
        use super::*;
        #[test]
        fn put_before_first() {
            assert_eq!(change_order(&vec![1, 2, 3, 4], 0, 1), vec![2, 1, 3, 4]);
        }
        #[test]
        fn put_last_before_first() {
            assert_eq!(change_order(&vec![1, 2, 3, 4], 0, 3), vec![4, 1, 2, 3]);
        }
        #[test]
        fn put_first_before_second() {
            assert_eq!(change_order(&vec![1, 2, 3, 4], 1, 0), vec![1, 2, 3, 4]);
        }
        #[test]
        fn put_before_second() {
            assert_eq!(change_order(&vec![1, 2, 3, 4], 1, 2), vec![1, 3, 2, 4]);
        }
        #[test]
        fn put_last_before_second() {
            assert_eq!(change_order(&vec![1, 2, 3, 4], 1, 3), vec![1, 4, 2, 3]);
        }
        #[test]
        fn put_first_before_last() {
            assert_eq!(change_order(&vec![1, 2, 3, 4], 3, 0), vec![2, 3, 1, 4]);
        }
        #[test]
        fn put_fourth_before_fourth() {
            assert_eq!(change_order(&vec![1, 2, 3, 4], 3, 3), vec![1, 2, 3, 4]);
        }
        #[test]
        fn put_first_before_first() {
            assert_eq!(change_order(&vec![1, 2, 3, 4], 3, 3), vec![1, 2, 3, 4]);
        }
        #[test]
        fn test_change_order_move_first() {
            assert_eq!(change_order(&vec![1, 2, 3], 1, 0), vec![1, 2, 3])
        }
        #[test]
        fn test_change_order_move_middle() {
            assert_eq!(change_order(&vec![1, 2, 3], 0, 1), vec![2, 1, 3])
        }

        #[test]
        fn test_change_order_move_last() {
            assert_eq!(change_order(&vec![1, 2, 3], 0, 2), vec![3, 1, 2])
        }
        #[test]
        fn test_change_order_move_first_before_last() {
            assert_eq!(change_order(&vec![1, 2, 3], 2, 0), vec![2, 1, 3])
        }
        #[test]
        fn test_change_order_move_middle_before_last() {
            assert_eq!(change_order(&vec![1, 2, 3], 2, 1), vec![1, 2, 3])
        }
    }
}
