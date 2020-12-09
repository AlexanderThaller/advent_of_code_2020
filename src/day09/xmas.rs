use itertools::Itertools;
use rayon::prelude::*;
use thiserror::Error;

#[allow(clippy::empty_enum)]
#[derive(Debug, Error)]
pub enum Error {}

pub struct Xmas {
    preamble: Vec<usize>,
    previous_count: usize,
    to_check: Vec<usize>,
}

impl Xmas {
    pub fn new(input: Vec<usize>, previous_count: usize) -> Self {
        let preamble = input.iter().take(previous_count).cloned().collect();
        let to_check = input.into_iter().skip(previous_count).collect();

        Self {
            previous_count,
            preamble,
            to_check,
        }
    }

    pub fn find_invalid(&mut self) -> Option<usize> {
        let to_check = self.to_check.clone();

        to_check.iter().find_map(|n| {
            if self.is_valid_number(*n) {
                None
            } else {
                Some(*n)
            }
        })
    }

    fn is_valid_number(&mut self, n: usize) -> bool {
        let found = self.find_number(n);

        if found {
            self.preamble.push(n);
        }

        found
    }

    fn find_number(&self, n: usize) -> bool {
        self.preamble
            .iter()
            .rev()
            .take(self.previous_count)
            .tuple_combinations()
            .any(|(first, second)| *first + *second == n)
    }

    pub fn find_contiguous_set(mut self, invalid_number: usize) -> Option<Vec<usize>> {
        self.preamble.append(&mut self.to_check);

        (0..self.preamble.len())
            .collect::<Vec<_>>()
            .par_iter()
            .find_map_first(|candidate_index| {
                self.check_candidate(invalid_number, *candidate_index)
            })
    }

    fn check_candidate(&self, invalid_number: usize, candidate_index: usize) -> Option<Vec<usize>> {
        let mut tmp = Vec::new();
        for index in candidate_index..self.preamble.len() {
            tmp.push(self.preamble[index]);

            let sum = tmp.iter().sum::<usize>();

            if sum == invalid_number {
                return Some(tmp);
            }

            if sum > invalid_number {
                break;
            }
        }

        None
    }
}

#[cfg(test)]
mod test {
    const FIRST_EXAMPLE: [usize; 25] = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    ];

    #[test]
    fn first_example_one_step_valid() {
        const INPUT: [usize; 2] = [26, 49];
        let mut xmas = super::Xmas::new(FIRST_EXAMPLE.to_vec(), 25);

        for input in &INPUT {
            let got = xmas.is_valid_number(*input);
            assert!(got);
        }
    }

    #[test]
    fn first_example_one_step_notvalid() {
        const INPUT: [usize; 2] = [100, 50];
        let mut xmas = super::Xmas::new(FIRST_EXAMPLE.to_vec(), 25);

        for input in &INPUT {
            let got = xmas.is_valid_number(*input);
            assert!(!got);
        }
    }

    #[test]
    fn first_example_two_step_valid() {
        const INPUT: [usize; 2] = [45, 64];
        let mut xmas = super::Xmas::new(FIRST_EXAMPLE.to_vec(), 25);

        for input in &INPUT {
            let got = xmas.is_valid_number(*input);
            assert!(got);
        }
    }

    #[test]
    fn second_example_find_invalid() {
        const INPUT: [usize; 20] = [
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        let mut xmas = super::Xmas::new(INPUT.to_vec(), 5);

        let expected = Some(127);

        let got = xmas.find_invalid();

        assert_eq!(expected, got);
    }

    #[test]
    fn second_example_find_contiguous_set() {
        const INPUT: [usize; 20] = [
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        const INVALID_NUMBER: usize = 127;

        let expected = Some(vec![15, 25, 47, 40]);

        let xmas = super::Xmas::new(INPUT.to_vec(), 5);
        let got = xmas.find_contiguous_set(INVALID_NUMBER);

        assert_eq!(expected, got);
    }
}
