use thiserror::Error;

mod xmas;

use xmas::Xmas;

#[allow(clippy::empty_enum)]
#[derive(Debug, Error)]
pub enum Error {
    #[error("found invalid number in input: {0}")]
    InvalidNumberInInput(std::num::ParseIntError),

    #[error("did not find invalid number in input")]
    NoInvalidNumberFound,
}

pub fn run() -> Result<(), Error> {
    println!("template::part_1: first invalid number = {}", part_1()?);
    println!("template::part_2: weakness = {}", part_2()?);

    Ok(())
}

pub fn part_1() -> Result<usize, Error> {
    let input = include_str!("input.txt")
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()
        .map_err(Error::InvalidNumberInInput)?;

    let mut xmas = Xmas::new(input, 25);
    let invalid = xmas.find_invalid().ok_or(Error::NoInvalidNumberFound)?;

    Ok(invalid)
}

pub fn part_2() -> Result<usize, Error> {
    let input = include_str!("input.txt")
        .lines()
        .map(str::parse)
        .collect::<Result<Vec<_>, _>>()
        .map_err(Error::InvalidNumberInInput)?;

    let mut xmas = Xmas::new(input, 25);
    let invalid = xmas.find_invalid().ok_or(Error::NoInvalidNumberFound)?;

    let mut contiguous_set = xmas.find_contiguous_set(invalid);
    contiguous_set.sort_unstable();

    let weakness = contiguous_set[0] + contiguous_set[contiguous_set.len() - 1];

    Ok(weakness)
}

#[cfg(test)]
mod test {
    #[test]
    fn part_1() {
        let expected = 41_682_220;
        let got = super::part_1().unwrap();

        assert_eq!(expected, got)
    }

    #[test]
    fn part_2() {
        let expected = 5388976;
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
