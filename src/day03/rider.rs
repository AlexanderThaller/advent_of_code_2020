use coordinate::Coordinate;
use map::Map;
use tile::Tile;

#[derive(Default)]
pub struct Toboggan {
    position: Coordinate,
    pub trees_seen: usize,

    map: Map,
}

pub fn new(map: Map) -> Toboggan {
    Toboggan {
        position: (0, 0).into(),
        trees_seen: 0,

        map,
    }
}

impl Toboggan {
    pub fn ride(&mut self) {
        loop {
            self.step();

            match self.map.get_tile(&self.position) {
                Some(tile) => {
                    if tile == &Tile::Tree {
                        self.trees_seen += 1;
                    }
                }

                // Reached bottom of map
                None => break,
            }
        }
    }

    fn step(&mut self) {
        self.position = self.position.step_right(3).step_down(1)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn ride() {
        let input = include_str!("input_test.txt").parse().expect("invalid map");
        let mut rider = super::new(input);
        rider.ride();

        let got = rider.trees_seen;
        let expected = 7;

        assert_eq!(expected, got);
    }
}

#[cfg(test)]
mod bench {
    use test::Bencher;

    #[bench]
    fn part_1(b: &mut Bencher) {
        let input: super::Map = include_str!("input.txt").parse().expect("invalid map");

        b.iter(|| {
            let mut rider = super::new(input.clone());
            rider.ride();
        });
    }
}

pub mod map {
    use std::{
        collections::HashMap,
        convert::TryInto,
    };
    use thiserror::Error;

    use super::{
        coordinate::Coordinate,
        tile::Tile,
    };

    #[derive(Debug, Error)]
    pub enum Error {
        #[error("invalid tile found: {0}")]
        InvalidTile(super::tile::Error),
    }

    #[derive(Debug, Default, Clone)]
    pub struct Map {
        entries: HashMap<Coordinate, Tile>,
    }

    impl std::str::FromStr for Map {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut entries = HashMap::default();

            for (y, line) in s.lines().enumerate() {
                for (x, ch) in line.chars().enumerate() {
                    let tile = ch.try_into().map_err(Error::InvalidTile)?;
                    entries.insert((x, y).into(), tile);
                }
            }

            Ok(Self { entries })
        }
    }

    impl std::iter::IntoIterator for Map {
        type Item = (Coordinate, Tile);
        type IntoIter = std::collections::hash_map::IntoIter<Coordinate, Tile>;

        fn into_iter(self) -> Self::IntoIter {
            self.entries.into_iter()
        }
    }

    impl From<Vec<(Coordinate, Tile)>> for Map {
        fn from(vec: Vec<(Coordinate, Tile)>) -> Self {
            let entries = vec.into_iter().collect();

            Self { entries }
        }
    }

    impl Map {
        pub fn get_tile(&self, coordinate: &Coordinate) -> Option<&Tile> {
            let map_width = self.max_x() + 1;
            let max_y = self.max_y();

            let mut check_coordinate = *coordinate;

            loop {
                if let Some(tile) = self.entries.get(&check_coordinate) {
                    return Some(tile);
                } else {
                    if check_coordinate.y > max_y {
                        return None;
                    }

                    check_coordinate = check_coordinate.step_left(map_width)
                }
            }
        }

        pub fn max_y(&self) -> usize {
            self.entries.keys().map(|c| c.y).max().unwrap_or_default()
        }

        pub fn max_x(&self) -> usize {
            self.entries.keys().map(|c| c.x).max().unwrap_or_default()
        }
    }

    #[cfg(test)]
    mod test {
        use super::{
            Map,
            Tile,
        };

        mod from_str {
            use super::Tile;
            use std::str::FromStr;

            #[test]
            fn minimal() {
                const INPUT: &str = ".";
                let mut expected = vec![((0, 0).into(), Tile::Air)];
                expected.sort();

                let mut got = super::Map::from_str(INPUT)
                    .expect("invalid input")
                    .into_iter()
                    .collect::<Vec<_>>();
                got.sort();

                assert_eq!(expected, got);
            }

            #[test]
            fn multiline() {
                const INPUT: &str = ".\n#";
                let mut expected = vec![((0, 0).into(), Tile::Air), ((0, 1).into(), Tile::Tree)];
                expected.sort();

                let mut got = super::Map::from_str(INPUT)
                    .expect("invalid input")
                    .into_iter()
                    .collect::<Vec<_>>();
                got.sort();

                assert_eq!(expected, got);
            }

            #[test]
            fn multi_dimension() {
                const INPUT: &str = ".#\n#.";
                let mut expected = vec![
                    ((0, 0).into(), Tile::Air),
                    ((1, 0).into(), Tile::Tree),
                    ((0, 1).into(), Tile::Tree),
                    ((1, 1).into(), Tile::Air),
                ];
                expected.sort();

                let mut got = super::Map::from_str(INPUT)
                    .expect("invalid input")
                    .into_iter()
                    .collect::<Vec<_>>();
                got.sort();

                assert_eq!(expected, got);
            }
        }

        mod get_tile {
            use super::Tile;

            #[test]
            fn single_column_no_trackback() {
                let map: super::Map = vec![((0, 0).into(), Tile::Air)].into();
                let expected = Some(&Tile::Air);
                let got = map.get_tile(&(0, 0).into());

                assert_eq!(expected, got);
            }

            #[test]
            fn single_column_trackback() {
                let map: super::Map = vec![((0, 0).into(), Tile::Air)].into();
                let expected = Some(&Tile::Air);
                let got = map.get_tile(&(1, 0).into());

                assert_eq!(expected, got);
            }
        }
    }
}

mod coordinate {
    #[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Default)]
    pub struct Coordinate {
        pub x: usize,
        pub y: usize,
    }

    impl From<(usize, usize)> for Coordinate {
        fn from(tuple: (usize, usize)) -> Self {
            Self {
                x: tuple.0,
                y: tuple.1,
            }
        }
    }

    impl Coordinate {
        pub fn step_right(self, steps: usize) -> Self {
            Self {
                x: self.x + steps,
                y: self.y,
            }
        }

        pub fn step_left(self, steps: usize) -> Self {
            Self {
                x: self.x - steps,
                y: self.y,
            }
        }

        pub fn step_down(self, steps: usize) -> Self {
            Self {
                x: self.x,
                y: self.y + steps,
            }
        }
    }
}

mod tile {
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum Error {
        #[error("don't know how to parse {0:?}")]
        UnkownTile(char),
    }

    #[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
    pub enum Tile {
        Tree,
        Air,
    }

    impl std::convert::TryFrom<char> for Tile {
        type Error = Error;

        fn try_from(c: char) -> Result<Self, Self::Error> {
            match c {
                '#' => Ok(Self::Tree),
                '.' => Ok(Self::Air),
                _ => Err(Error::UnkownTile(c)),
            }
        }
    }
}