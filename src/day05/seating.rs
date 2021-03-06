use thiserror::Error;

#[allow(clippy::empty_enum)]
#[derive(Debug, Error)]
pub enum Error {}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Seat {
    pub id: usize,

    pub row: usize,
    pub column: usize,
}

impl From<&str> for Seat {
    fn from(s: &str) -> Self {
        let row = get_row(s);
        let column = get_column(s);
        let id = (row * 8) + column;

        Self { row, column, id }
    }
}

fn get_row(s: &str) -> usize {
    parse_partition(s, 'F', 'B', 127)
}

fn get_column(s: &str) -> usize {
    parse_partition(s, 'L', 'R', 7)
}

fn parse_partition(s: &str, choose_upper: char, choose_lower: char, max_value: usize) -> usize {
    let mut values = (0..=max_value).collect::<Vec<_>>();

    for ch in s.chars() {
        if ch != choose_upper && ch != choose_lower {
            continue;
        }

        let middle_index = values.len() / 2;
        let lower = values.split_off(middle_index);

        // Don't need to check choose_upper as columns.split_off will already update
        // values with the upper half of the slice so we don't need to do anything here.
        if ch == choose_lower {
            values = lower
        }
    }

    values
        .into_iter()
        .min()
        .expect("values should always contain at least one entry at this point")
}

#[cfg(test)]
mod test {
    use super::{
        get_column,
        get_row,
        Seat,
    };

    const INPUT_PART1_EXAMPLE_1: &str = "BFFFBBFRRR";
    const INPUT_PART1_EXAMPLE_2: &str = "FFFBBBFRRR";
    const INPUT_PART1_EXAMPLE_3: &str = "BBFFBBFRLL";

    mod get_row {
        use super::*;

        #[test]
        fn part_1_example_1() {
            let expected = 70;
            let got = get_row(INPUT_PART1_EXAMPLE_1);

            assert_eq!(expected, got);
        }

        #[test]
        fn part_1_example_2() {
            let expected = 14;
            let got = get_row(INPUT_PART1_EXAMPLE_2);

            assert_eq!(expected, got);
        }

        #[test]
        fn part_1_example_3() {
            let expected = 102;
            let got = get_row(INPUT_PART1_EXAMPLE_3);

            assert_eq!(expected, got);
        }
    }

    mod get_column {
        use super::*;

        #[test]
        fn part_1_example_1() {
            let expected = 7;
            let got = get_column(INPUT_PART1_EXAMPLE_1);

            assert_eq!(expected, got);
        }

        #[test]
        fn part_1_example_2() {
            let expected = 7;
            let got = get_column(INPUT_PART1_EXAMPLE_2);

            assert_eq!(expected, got);
        }

        #[test]
        fn part_1_example_3() {
            let expected = 4;
            let got = get_column(INPUT_PART1_EXAMPLE_3);

            assert_eq!(expected, got);
        }
    }

    mod from_str {
        use super::*;

        #[test]
        fn part_1_example_1() {
            let expected = Seat {
                row: 70,
                column: 7,
                id: 567,
            };

            let got = Seat::from(INPUT_PART1_EXAMPLE_1);

            assert_eq!(expected, got);
        }

        #[test]
        fn part_1_example_2() {
            let expected = Seat {
                row: 14,
                column: 7,
                id: 119,
            };

            let got = Seat::from(INPUT_PART1_EXAMPLE_2);

            assert_eq!(expected, got);
        }

        #[test]
        fn part_1_example_3() {
            let expected = Seat {
                row: 102,
                column: 4,
                id: 820,
            };

            let got = Seat::from(INPUT_PART1_EXAMPLE_3);

            assert_eq!(expected, got);
        }
    }
}
