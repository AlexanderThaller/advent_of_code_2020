use thiserror::Error;

mod rider;

#[derive(Debug, Error)]
pub enum Error {
    #[error("can not read map: {0}")]
    MapError(#[from] rider::map::Error),
}

pub fn run() -> Result<(), Error> {
    part_1()?;
    part_2()?;

    Ok(())
}

pub fn part_1() -> Result<usize, Error> {
    let input = include_str!("input.txt").parse()?;
    let (right, down) = (3, 1);

    let mut rider = rider::new(input);
    rider.ride(right, down);

    println!("day_03::part_1: trees_seen = {}", rider.trees_seen);

    Ok(rider.trees_seen)
}

pub fn part_2() -> Result<usize, Error> {
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let input = include_str!("input.txt").parse()?;
    let mut rider = rider::new(input);

    let trees_seen = slopes
        .into_iter()
        .map(|(right, down)| {
            rider.ride(right, down);
            let trees_seen = rider.trees_seen;
            rider.reset();

            trees_seen
        })
        .product::<usize>();

    println!("day_03::part_2: total trees_seen = {}", trees_seen);

    Ok(trees_seen)
}

#[cfg(test)]
mod test {
    #[test]
    fn part_1() {
        let got = super::part_1().expect("error while running");
        let expected = 262;

        assert_eq!(expected, got);
    }

    #[test]
    fn part_2() {
        let got = super::part_2().expect("error while running");
        let expected = 2_698_900_776;

        assert_eq!(expected, got);
    }
}

#[cfg(test)]
mod bench {
    use test::Bencher;

    #[bench]
    fn part_1(b: &mut Bencher) {
        b.iter(|| {
            let _ = super::part_1();
        });
    }

    #[bench]
    fn part_2(b: &mut Bencher) {
        b.iter(|| {
            let _ = super::part_2();
        });
    }
}
