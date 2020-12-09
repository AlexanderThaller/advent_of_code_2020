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
    println!("template::part_2: value = {}", part_2()?);

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
    Ok(0)
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
        let expected = 0;
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
