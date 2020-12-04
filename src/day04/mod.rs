use thiserror::Error;

mod passport;
mod passport_strict;

use passport::Passports;
use passport_strict::Passports as PassportsStrict;

#[derive(Debug, Error)]
pub enum Error {}

pub fn run() -> Result<(), Error> {
    println!("day_03::part_1: valid passports = {}", part_1()?);
    println!("day_03::part_2: valid passports = {}", part_2()?);

    Ok(())
}

pub fn part_1() -> Result<usize, Error> {
    const INPUT: &str = include_str!("input.txt");
    let valid_passports = INPUT.parse::<Passports>().unwrap().len();

    Ok(valid_passports)
}

pub fn part_2() -> Result<usize, Error> {
    const INPUT: &str = include_str!("input.txt");
    let valid_passports = INPUT.parse::<PassportsStrict>().unwrap().len();

    Ok(valid_passports)
}

#[cfg(test)]
mod test {
    #[test]
    fn part_1() {
        let expected = 190;
        let got = super::part_1().unwrap();

        assert_eq!(expected, got)
    }

    #[test]
    fn part_2() {
        let expected = 121;
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
