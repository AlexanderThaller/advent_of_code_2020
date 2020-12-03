pub mod coordinate;
pub mod map;
pub mod tile;

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
    pub fn ride(&mut self, right: usize, down: usize) {
        loop {
            self.step(right, down);

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

    fn step(&mut self, right: usize, down: usize) {
        self.position = self.position.step_right(right).step_down(down)
    }

    pub fn reset(&mut self) {
        self.position = Self::default().position;
        self.trees_seen = Self::default().trees_seen;
    }
}

#[cfg(test)]
mod test {
    use super::{
        new,
        Map,
    };

    mod ride {
        #[test]
        fn input_test_part1() {
            let input = include_str!("../input_test.txt")
                .parse()
                .expect("invalid map");
            let mut rider = super::new(input);
            rider.ride(3, 1);

            let got = rider.trees_seen;
            let expected = 7;

            assert_eq!(expected, got);
        }

        #[test]
        fn input_test_part2() {
            let input: super::Map = include_str!("../input_test.txt")
                .parse()
                .expect("invalid map");
            let slopes = vec![
                ((1, 1), 2),
                ((3, 1), 7),
                ((5, 1), 3),
                ((7, 1), 4),
                ((1, 2), 2),
            ];

            let mut rider = super::new(input);

            for ((right, down), expected) in slopes {
                rider.ride(right, down);

                let got = rider.trees_seen;

                if expected != got {
                    dbg!(right);
                    dbg!(down);
                }

                assert_eq!(expected, got);

                rider.reset()
            }
        }
    }
}

#[cfg(test)]
mod bench {
    use super::{
        new,
        Map,
    };

    mod ride {
        use test::Bencher;

        #[bench]
        fn input_test_part1(b: &mut Bencher) {
            let input = include_str!("../input_test.txt")
                .parse()
                .expect("invalid map");

            let mut rider = super::new(input);

            b.iter(|| {
                rider.ride(3, 1);
                rider.reset();
            });
        }

        #[bench]
        fn input_test_part2(b: &mut Bencher) {
            let input: super::Map = include_str!("../input_test.txt")
                .parse()
                .expect("invalid map");
            let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

            let mut rider = super::new(input);

            b.iter(|| {
                for (right, down) in &slopes {
                    rider.ride(*right, *down);
                    rider.reset();
                }
            });
        }
    }
}
