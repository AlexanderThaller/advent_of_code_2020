use std::{
    collections::HashSet,
    ops::BitAnd,
};

pub fn count_yes(s: &str) -> usize {
    s.split("\n\n")
        .map(|group| {
            group
                .lines()
                .flat_map(str::chars)
                .collect::<HashSet<_>>()
                .len()
        })
        .sum()
}

pub fn count_yes_all(s: &str) -> usize {
    s.split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.chars().collect::<HashSet<_>>())
                .fold_first(|set1, set2| set1.bitand(&set2))
                .map_or(0, |group| group.len())
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    mod count_yes {
        #[test]
        fn example_part1() {
            let input = include_str!("example_part1.txt");
            let expected = 11;
            let got = super::count_yes(input);

            assert_eq!(expected, got);
        }
    }

    mod count_yes_all {
        #[test]
        fn example_part1_group_1() {
            let input = "abc";
            let expected = 3;
            let got = super::count_yes_all(input);

            assert_eq!(expected, got);
        }

        #[test]
        fn example_part1_group_2() {
            let input = "a\nb\nc";
            let expected = 0;
            let got = super::count_yes_all(input);

            assert_eq!(expected, got);
        }

        #[test]
        fn example_part1_group_3() {
            let input = "ab\nac";
            let expected = 1;
            let got = super::count_yes_all(input);

            assert_eq!(expected, got);
        }

        #[test]
        fn example_part1_group_4() {
            let input = "a\na\na\na";
            let expected = 1;
            let got = super::count_yes_all(input);

            assert_eq!(expected, got);
        }

        #[test]
        fn example_part1_group_5() {
            let input = "b";
            let expected = 1;
            let got = super::count_yes_all(input);

            assert_eq!(expected, got);
        }

        #[test]
        fn example_part1() {
            let input = include_str!("example_part1.txt");
            let expected = 6;
            let got = super::count_yes_all(input);

            assert_eq!(expected, got);
        }
    }
}
