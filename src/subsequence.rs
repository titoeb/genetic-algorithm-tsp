use crate::utils::get_random_elem_from_range;

/// The `Subsequence`-object only stores the indexes of a potential subsequences. Then based on a sequence, operations
/// on that subsequence can be applied.
#[derive(Debug)]
pub struct Subsequence {
    /// Where does the subsequence start?
    pub start_index: usize,
    /// How long is the subsequence?
    pub length: usize,
}

impl Subsequence {
    /// Create a new subseqence
    ///
    /// # Arguments
    ///
    /// * `start_index` - Where should the subsequence start?
    /// * `length` - How long is the subsequence?
    pub fn new(start_index: usize, length: usize) -> Self {
        Subsequence {
            start_index,
            length,
        }
    }
    /// Create a new, random subsequence.
    ///
    /// # Arguments
    ///
    /// * `len_sequence` - What is the len of the actual sequence that should be subsequenced?
    pub fn random_subsequence(len_sequence: usize) -> Self {
        let start_index = get_random_elem_from_range(0..(len_sequence - 2));
        Subsequence {
            start_index,
            length: get_random_elem_from_range(1..(len_sequence - start_index - 1)),
        }
    }
    /// Based on an actual sequence, get all elements that are in the subsequence
    ///
    /// # Arguments
    ///
    /// * `sequence` - The actual sequence that should be subsequenced
    ///
    pub fn get_values_in<'a>(&self, sequence: &'a [usize]) -> Option<&'a [usize]> {
        if self.start_index + self.length <= sequence.len() {
            Some(&sequence[self.start_index..(self.start_index + self.length)])
        } else {
            None
        }
    }
    /// Based on an actual sequence, get all elements that come before the subsequence
    ///
    /// # Arguments
    ///
    /// * `sequence` - The actual sequence that should be subsequenced
    ///
    pub fn get_values_before<'a>(&self, sequence: &'a [usize]) -> Option<&'a [usize]> {
        if self.start_index <= sequence.len() {
            Some(&sequence[..self.start_index])
        } else {
            None
        }
    }
    /// Based on an actual sequence, get all elements that come after the subsequence
    ///
    /// # Arguments
    ///
    /// * `sequence` - The actual sequence that should be subsequenced
    ///
    pub fn get_values_after<'a>(&self, sequence: &'a [usize]) -> Option<&'a [usize]> {
        if self.start_index + self.length <= sequence.len() {
            Some(&sequence[(self.start_index + self.length)..])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod test_random_subsequence {
        use super::*;
        #[test]
        #[test]
        #[test]
        #[test]
        fn test_max_10() {
            let max_value = 10;
            let random_subsequence = Subsequence::random_subsequence(max_value);
            assert!(random_subsequence.start_index < max_value);
            assert!(random_subsequence.length < max_value - random_subsequence.start_index);
            assert!(random_subsequence.start_index + random_subsequence.length < max_value);
        }
        #[test]
        #[test]
        #[test]
        #[test]
        fn test_max_2() {
            let max_value = 2;
            let random_subsequence = Subsequence::random_subsequence(max_value);
            assert!(random_subsequence.start_index < max_value);
            assert!(random_subsequence.length < max_value - random_subsequence.start_index);
            assert!(random_subsequence.start_index + random_subsequence.length < max_value);
        }
    }
    mod test_get_values_in_subsequence {
        use super::*;
        #[test]
        fn partial_subsequence() {
            let sequence: Vec<usize> = (0..10).collect();
            let subsequence = Subsequence {
                start_index: 3,
                length: 4,
            };
            assert_eq!(subsequence.get_values_in(&sequence), Some(&sequence[3..=6]))
        }
        #[test]
        fn full_subsequence() {
            let sequence: Vec<usize> = (0..10).collect();
            let subsequence = Subsequence {
                start_index: 0,
                length: 10,
            };
            assert_eq!(subsequence.get_values_in(&sequence), Some(&sequence[0..]))
        }
        #[test]
        fn too_short() {
            let sequence: Vec<usize> = (0..10).collect();
            let subsequence = Subsequence {
                start_index: 5,
                length: 12,
            };
            assert_eq!(subsequence.get_values_in(&sequence), None)
        }
    }
    mod test_get_values_before_subsequence {
        use super::*;
        #[test]
        fn partial_subsequence() {
            let sequence: Vec<usize> = (0..10).collect();
            let subsequence = Subsequence {
                start_index: 3,
                length: 4,
            };
            assert_eq!(
                subsequence.get_values_before(&sequence),
                Some(&sequence[0..3])
            )
        }
        #[test]
        fn full_subsequence() {
            let sequence: Vec<usize> = (0..10).collect();
            let subsequence = Subsequence {
                start_index: 0,
                length: 10,
            };
            assert_eq!(
                subsequence.get_values_before(&sequence),
                Some(&Vec::<usize>::new()[0..])
            )
        }
        #[test]
        fn too_short_for_sequence() {
            let sequence: Vec<usize> = (0..10).collect();
            let subsequence = Subsequence {
                start_index: 5,
                length: 12,
            };
            assert_eq!(
                subsequence.get_values_before(&sequence),
                Some(&sequence[0..5])
            )
        }
        #[test]
        fn too_short_for_before_subsequence() {
            let sequence: Vec<usize> = (0..10).collect();
            let subsequence = Subsequence {
                start_index: 11,
                length: 12,
            };
            assert_eq!(subsequence.get_values_before(&sequence), None)
        }
    }
    mod test_get_values_after_subsequence {
        use super::*;
        #[test]
        fn partial_subsequence() {
            let sequence: Vec<usize> = (0..10).collect();
            let subsequence = Subsequence {
                start_index: 3,
                length: 4,
            };
            assert_eq!(
                subsequence.get_values_after(&sequence),
                Some(&sequence[7..10])
            )
        }
        #[test]
        fn full_subsequence() {
            let sequence: Vec<usize> = (0..10).collect();
            let subsequence = Subsequence {
                start_index: 0,
                length: 10,
            };
            assert_eq!(
                subsequence.get_values_after(&sequence),
                Some(&Vec::<usize>::new()[0..])
            )
        }
        #[test]
        fn too_short() {
            let sequence: Vec<usize> = (0..10).collect();
            let subsequence = Subsequence {
                start_index: 5,
                length: 12,
            };
            assert_eq!(subsequence.get_values_after(&sequence), None)
        }
    }
}
