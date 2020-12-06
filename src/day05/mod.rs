use thiserror::Error;

mod seating;

use seating::Seat;

#[allow(clippy::empty_enum)]
#[derive(Debug, Error)]
pub enum Error {}

pub fn run() -> Result<(), Error> {
    println!("day_05::part_1: maximum seat id = {}", part_1()?);
    println!("day_05::part_2: seat = {:?}", part_2()?);

    Ok(())
}

pub fn part_1() -> Result<usize, Error> {
    let max_id = include_str!("input.txt")
        .lines()
        .map(|line| Seat::from(line).id)
        .max()
        .unwrap();

    Ok(max_id)
}

#[allow(dead_code)]
pub fn part_2() -> Result<usize, Error> {
    let mut seats = include_str!("input.txt")
        .lines()
        .map(|line| Seat::from(line).id)
        .collect::<Vec<_>>();

    seats.sort_unstable();

    let seat_hole = seats
        .iter()
        .zip(seats.iter().skip(1))
        .find(|(previous, now)| **previous != 0 && (now.saturating_sub(1)) != **previous)
        .unwrap();

    let seat_id = seat_hole.0 + 1;

    Ok(seat_id)
}

#[cfg(test)]
mod test {
    #[test]
    fn part_1() {
        let expected = 848;
        let got = super::part_1().unwrap();

        assert_eq!(expected, got)
    }

    #[test]
    fn part_2() {
        let expected = 682;
        let got = super::part_2().unwrap();

        assert_eq!(expected, got)
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
