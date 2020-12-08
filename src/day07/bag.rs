use std::collections::{
    HashMap,
    HashSet,
};
use thiserror::Error;

#[allow(clippy::empty_enum)]
#[derive(Debug, Error)]
pub enum Error {
    #[error("invalid input")]
    InvalidInput,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Bags(Vec<Bag>);

impl Bags {
    fn find_containers(&self, for_bag_color: &str) -> Vec<&Bag> {
        self.0
            .iter()
            .filter(|bag| bag.can_contain.contains_key(for_bag_color))
            .collect()
    }

    fn find_bag(&self, for_bag_color: &str) -> Option<&Bag> {
        self.0.iter().find(|bag| bag.color == for_bag_color)
    }

    pub fn find_all_containers(&self, for_bag_color: &str) -> HashSet<String> {
        self.find_containers(for_bag_color)
            .into_iter()
            .flat_map(|bag| {
                let mut parents = self.find_all_containers(&bag.color);
                parents.insert(bag.color.clone());

                parents
            })
            .collect()
    }

    pub fn must_contain(&self, for_bag_color: &str) -> Vec<(String, usize)> {
        let mut out = Vec::new();

        if let Some(bag) = self.find_bag(for_bag_color) {
            for (color, count) in &bag.can_contain {
                out.push((color.clone(), *count));
                out.append(&mut self.must_contain_recurse(color, *count))
            }
        }

        out
    }

    fn must_contain_recurse(
        &self,
        for_bag_color: &str,
        parent_count: usize,
    ) -> Vec<(String, usize)> {
        let mut out = Vec::new();

        if let Some(bag) = self.find_bag(for_bag_color) {
            for (color, count) in &bag.can_contain {
                let new_count = *count * parent_count;
                out.push((color.clone(), new_count));
                out.append(&mut self.must_contain_recurse(color, new_count))
            }
        }

        out
    }
}

impl From<&str> for Bags {
    fn from(s: &str) -> Self {
        Self(s.lines().filter_map(|line| line.parse().ok()).collect())
    }
}

impl From<Vec<Bag>> for Bags {
    fn from(v: Vec<Bag>) -> Self {
        Self(v)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Bag {
    color: String,
    can_contain: HashMap<String, usize>,
}

impl std::str::FromStr for Bag {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s.split_ascii_whitespace().collect::<Vec<_>>();

        match split.as_slice() {
            [modifier, color, "bags", "contain", contains @ ..] => {
                let contains = contains.join(" ");
                let can_contain = contains
                    .split(',')
                    .map(str::trim)
                    .map(|s| s.split_ascii_whitespace().collect::<Vec<_>>())
                    .filter_map(|v| match v.as_slice() {
                        [amount, modifier_color @ .., "bag"]
                        | [amount, modifier_color @ .., "bag."]
                        | [amount, modifier_color @ .., "bags."]
                        | [amount, modifier_color @ .., "bags"] => {
                            if amount == &"no" {
                                None
                            } else {
                                Some((modifier_color.join(" "), amount.parse().unwrap()))
                            }
                        }

                        _ => {
                            unreachable!()
                        }
                    })
                    .collect::<HashMap<_, _>>();

                Ok(Self {
                    color: format!("{} {}", modifier, color),
                    can_contain,
                })
            }

            [] => Err(Error::InvalidInput),

            _ => {
                unreachable!()
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod must_contain {
        #[test]
        fn contain() {
            const INPUT: &str =
                "shiny gold bags contain 1 muted blue bag\nmuted blue bags contain no other bags.";

            let expected = vec![("muted blue".into(), 1)];
            let got = super::Bags::from(INPUT).must_contain("shiny gold");

            assert_eq!(expected, got)
        }

        #[test]
        fn contain_contain() {
            const INPUT: &str = "shiny gold bags contain 1 muted blue bag\nmuted blue bags \
                                 contain 1 muted yellow bag\nmuted yellow bags contain no other \
                                 bags.";

            let expected = vec![("muted blue".into(), 1), ("muted yellow".into(), 1)];
            let got = super::Bags::from(INPUT).must_contain("shiny gold");

            assert_eq!(expected, got)
        }

        #[test]
        fn contain_contain_contain() {
            const INPUT: &str = "shiny gold bags contain 1 muted blue bag\nmuted blue bags \
                                 contain 1 muted yellow bag\nmuted yellow bags contain 1 muted \
                                 green bag\nmuted green bags contain no other bags.";

            let expected = vec![
                ("muted blue".into(), 1),
                ("muted yellow".into(), 1),
                ("muted green".into(), 1),
            ];
            let got = super::Bags::from(INPUT).must_contain("shiny gold");

            assert_eq!(expected, got)
        }

        #[test]
        fn example() {
            const INPUT: &str = include_str!("input_example.txt");
            let mut expected = vec![
                ("vibrant plum".into(), 2),
                ("faded blue".into(), 10),
                ("dotted black".into(), 12),
                ("dark olive".into(), 1),
                ("dotted black".into(), 4),
                ("faded blue".into(), 3),
            ];
            expected.sort();

            let expected_count = 32;

            let mut got = super::Bags::from(INPUT).must_contain("shiny gold");
            got.sort();

            let got_count = super::Bags::from(INPUT)
                .must_contain("shiny gold")
                .into_iter()
                .map(|(_, count)| count)
                .sum::<usize>();

            assert_eq!(expected, got);
            assert_eq!(expected_count, got_count)
        }

        #[test]
        fn example2() {
            const INPUT: &str = include_str!("input_example2.txt");
            let mut expected = vec![
                ("dark red".into(), 2),
                ("dark orange".into(), 4),
                ("dark yellow".into(), 8),
                ("dark green".into(), 16),
                ("dark blue".into(), 32),
                ("dark violet".into(), 64),
            ];
            expected.sort();

            let expected_count = 126;

            let mut got = super::Bags::from(INPUT).must_contain("shiny gold");
            got.sort();

            let got_count = super::Bags::from(INPUT)
                .must_contain("shiny gold")
                .into_iter()
                .map(|(_, count)| count)
                .sum::<usize>();

            assert_eq!(expected, got);
            assert_eq!(expected_count, got_count)
        }
    }

    mod find_all_containers {
        #[test]
        fn contain_contain() {
            const INPUT: &str = "muted yellow bags contain 1 shiny gold bag\nmuted blue bags \
                                 contain 1 muted yellow bag";

            let expected = 2;
            let got = super::Bags::from(INPUT)
                .find_all_containers("shiny gold")
                .len();

            assert_eq!(expected, got)
        }

        #[test]
        fn contain_contain_contain() {
            const INPUT: &str = "muted yellow bags contain 1 shiny gold bag\nmuted blue bags \
                                 contain 1 muted yellow bag\nmuted green bags contain 1 muted \
                                 blue bag";

            let expected = 3;
            let got = super::Bags::from(INPUT)
                .find_all_containers("shiny gold")
                .len();

            assert_eq!(expected, got)
        }

        #[test]
        fn example() {
            const INPUT: &str = include_str!("input_example.txt");
            let expected = 4;
            let got = super::Bags::from(INPUT)
                .find_all_containers("shiny gold")
                .len();

            assert_eq!(expected, got)
        }
    }

    mod bag_from_str {
        #[test]
        fn example_line_0() {
            const INPUT: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.";

            let expected = super::Bag {
                color: "light red".into(),
                can_contain: vec![("bright white".into(), 1), ("muted yellow".into(), 2)]
                    .into_iter()
                    .collect(),
            };

            let got = INPUT.parse().unwrap();

            assert_eq!(expected, got);
        }

        #[test]
        fn example_line_1() {
            const INPUT: &str =
                "dark orange bags contain 3 bright white bags, 4 muted yellow bags.";

            let expected = super::Bag {
                color: "dark orange".into(),
                can_contain: vec![("bright white".into(), 3), ("muted yellow".into(), 4)]
                    .into_iter()
                    .collect(),
            };

            let got = INPUT.parse().unwrap();

            assert_eq!(expected, got);
        }

        #[test]
        fn example_line_2() {
            const INPUT: &str = "bright white bags contain 1 shiny gold bag.";

            let expected = super::Bag {
                color: "bright white".into(),
                can_contain: vec![("shiny gold".into(), 1)].into_iter().collect(),
            };

            let got = INPUT.parse().unwrap();

            assert_eq!(expected, got);
        }

        #[test]
        fn example_line_3() {
            const INPUT: &str = "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.";

            let expected = super::Bag {
                color: "muted yellow".into(),
                can_contain: vec![("shiny gold".into(), 2), ("faded blue".into(), 9)]
                    .into_iter()
                    .collect(),
            };

            let got = INPUT.parse().unwrap();

            assert_eq!(expected, got);
        }

        #[test]
        fn example_line_4() {
            const INPUT: &str = "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.";

            let expected = super::Bag {
                color: "shiny gold".into(),
                can_contain: vec![("dark olive".into(), 1), ("vibrant plum".into(), 2)]
                    .into_iter()
                    .collect(),
            };

            let got = INPUT.parse().unwrap();

            assert_eq!(expected, got);
        }

        #[test]
        fn example_line_5() {
            const INPUT: &str = "dark olive bags contain 3 faded blue bags, 4 dotted black bags.";

            let expected = super::Bag {
                color: "dark olive".into(),
                can_contain: vec![("faded blue".into(), 3), ("dotted black".into(), 4)]
                    .into_iter()
                    .collect(),
            };

            let got = INPUT.parse().unwrap();

            assert_eq!(expected, got);
        }

        #[test]
        fn example_line_6() {
            const INPUT: &str = "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.";

            let expected = super::Bag {
                color: "vibrant plum".into(),
                can_contain: vec![("faded blue".into(), 5), ("dotted black".into(), 6)]
                    .into_iter()
                    .collect(),
            };

            let got = INPUT.parse().unwrap();

            assert_eq!(expected, got);
        }

        #[test]
        fn example_line_7() {
            const INPUT: &str = "faded blue bags contain no other bags.";

            let expected = super::Bag {
                color: "faded blue".into(),
                can_contain: vec![].into_iter().collect(),
            };

            let got = INPUT.parse().unwrap();

            assert_eq!(expected, got);
        }

        #[test]
        fn example_line_8() {
            const INPUT: &str = "dotted black bags contain no other bags.";

            let expected = super::Bag {
                color: "dotted black".into(),
                can_contain: vec![].into_iter().collect(),
            };

            let got = INPUT.parse().unwrap();

            assert_eq!(expected, got);
        }
    }
}
