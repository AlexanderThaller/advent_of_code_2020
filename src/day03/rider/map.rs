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
    max_coordinate: Coordinate,
}

impl std::str::FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut entries = HashMap::default();
        let mut max_coordinate = Coordinate::default();

        for (y, line) in s.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let tile = ch.try_into().map_err(Error::InvalidTile)?;

                let coordinate = (x, y).into();
                if coordinate > max_coordinate {
                    max_coordinate = coordinate
                }

                entries.insert(coordinate, tile);
            }
        }

        Ok(Self {
            entries,
            max_coordinate,
        })
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
        let entries = vec.into_iter().collect::<HashMap<_, _>>();

        let max_coordinate = entries
            .keys()
            .max()
            .unwrap_or(&Coordinate { x: 0, y: 0 })
            .to_owned();

        Self {
            entries,
            max_coordinate,
        }
    }
}

impl Map {
    pub fn get_tile(&self, coordinate: &Coordinate) -> Option<&Tile> {
        let max_y = self.max_y();
        if coordinate.y > max_y {
            return None;
        }

        let max_x = self.max_x();

        let check_coordinate = if coordinate.x > max_x {
            (coordinate.x % (max_x + 1), coordinate.y).into()
        } else {
            *coordinate
        };

        self.entries.get(&check_coordinate)
    }

    pub fn max_y(&self) -> usize {
        self.max_coordinate.y
    }

    pub fn max_x(&self) -> usize {
        self.max_coordinate.x
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

#[cfg(test)]
mod bench {
    use super::{
        Map,
        Tile,
    };

    mod from_str {
        use std::str::FromStr;
        use test::Bencher;

        #[bench]
        fn minimal(b: &mut Bencher) {
            const INPUT: &str = ".";

            b.iter(|| {
                let _ = super::Map::from_str(INPUT);
            })
        }

        #[bench]
        fn multiline(b: &mut Bencher) {
            const INPUT: &str = ".\n#";

            b.iter(|| {
                let _ = super::Map::from_str(INPUT);
            })
        }

        #[bench]
        fn multi_dimension(b: &mut Bencher) {
            const INPUT: &str = ".#\n#.";

            b.iter(|| {
                let _ = super::Map::from_str(INPUT);
            })
        }

        #[bench]
        fn test_input(b: &mut Bencher) {
            const INPUT: &str = include_str!("../input_test.txt");

            b.iter(|| {
                let _ = super::Map::from_str(INPUT);
            })
        }

        #[bench]
        fn input(b: &mut Bencher) {
            const INPUT: &str = include_str!("../input.txt");

            b.iter(|| {
                let _ = super::Map::from_str(INPUT);
            })
        }
    }

    mod get_tile {
        use super::Tile;
        use std::str::FromStr;
        use test::Bencher;

        #[bench]
        fn single_column_no_trackback(b: &mut Bencher) {
            let map: super::Map = vec![((0, 0).into(), Tile::Air)].into();

            b.iter(|| {
                let _ = map.get_tile(&(0, 0).into());
            })
        }

        #[bench]
        fn single_column_trackback(b: &mut Bencher) {
            let map: super::Map = vec![((0, 0).into(), Tile::Air)].into();

            b.iter(|| {
                let _ = map.get_tile(&(1, 0).into());
            })
        }

        #[bench]
        fn input(b: &mut Bencher) {
            const INPUT: &str = include_str!("../input.txt");
            let map = super::Map::from_str(INPUT).expect("invalid input");

            b.iter(|| {
                let _ = map.get_tile(&(0, 0).into());
            })
        }
    }
}
