use std::convert::TryInto;
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

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Map {
    entries: Vec<Vec<Tile>>,
    max_coordinate: Coordinate,
}

impl std::str::FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let entries = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| ch.try_into().map_err(Error::InvalidTile))
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(entries.into())
    }
}

impl From<Vec<Vec<Tile>>> for Map {
    fn from(entries: Vec<Vec<Tile>>) -> Self {
        let max_y = entries.len() - 1;
        let max_x = entries.get(0).map_or(0, |line| line.len() - 1);
        let max_coordinate = Coordinate { x: max_x, y: max_y };

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

        Some(&self.entries[check_coordinate.y][check_coordinate.x])
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
            let expected: super::Map = vec![vec![Tile::Air]].into();
            let got = super::Map::from_str(INPUT).expect("invalid input");

            assert_eq!(expected, got);
        }

        #[test]
        fn multiline() {
            const INPUT: &str = ".\n#";
            let expected: super::Map = vec![vec![Tile::Air], vec![Tile::Tree]].into();
            let got = super::Map::from_str(INPUT).expect("invalid input");

            assert_eq!(expected, got);
        }

        #[test]
        fn multi_dimension() {
            const INPUT: &str = ".#\n#.";

            let expected: super::Map =
                vec![vec![Tile::Air, Tile::Tree], vec![Tile::Tree, Tile::Air]].into();
            let got = super::Map::from_str(INPUT).expect("invalid input");

            assert_eq!(expected, got);
        }
    }

    mod get_tile {
        use super::Tile;

        #[test]
        fn single_column_no_trackback() {
            let map: super::Map = vec![vec![Tile::Air]].into();
            let expected = Some(&Tile::Air);
            let got = map.get_tile(&(0, 0).into());

            assert_eq!(expected, got);
        }

        #[test]
        fn single_column_trackback() {
            let map: super::Map = vec![vec![Tile::Air]].into();
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
            let map: super::Map = vec![vec![Tile::Air]].into();

            b.iter(|| {
                let _ = map.get_tile(&(0, 0).into());
            })
        }

        #[bench]
        fn single_column_trackback(b: &mut Bencher) {
            let map: super::Map = vec![vec![Tile::Air]].into();

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
