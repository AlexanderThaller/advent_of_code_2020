use thiserror::Error;

mod input;
mod password_policy;
mod tester;

use input::INPUT;
use password_policy::{
    sled_rental::PasswordPolicy as PasswordPolicySledRental,
    toboggan_rental::PasswordPolicy as PasswordPolicyTobogganRental,
};
use tester::{
    SledTester,
    TobogganTester,
};

#[cfg(test)]
const TEST_INPUT_SLED_RENTAL: [(&str, Result<bool, Error>); 3] = [
    ("1-3 a: abcde", Ok(true)),
    ("1-3 b: cdefg", Ok(false)),
    ("2-9 c: ccccccccc", Ok(true)),
];

#[cfg(test)]
const TEST_INPUT_TOBOGGAN_RENTAL: [(&str, Result<bool, Error>); 4] = [
    ("1-3 a: abcde", Ok(true)),
    ("1-3 b: cdefg", Ok(false)),
    ("2-9 c: ccccccccc", Ok(false)),
    // Case when the first index does not contain the character. In that case we want to check the
    // not_contains_index if the character exists. If it the character exists in the
    // not_contains_index the password is valid.
    ("2-9 c: cdccccccc", Ok(true)),
];

#[derive(Debug, Error, Eq, PartialEq)]
pub enum Error {
    #[error("missing policy in input")]
    MissingPolicy,

    #[error("invalid sled policy: {0}")]
    InvalidSledPolicy(password_policy::sled_rental::Error),

    #[error("invalid sled policy: {0}")]
    InvalidTobogganPolicy(password_policy::toboggan_rental::Error),

    #[error("missing password in input")]
    MissingPassword,
}

pub fn run() -> Result<(), Error> {
    part_1()?;
    part_2()?;

    Ok(())
}

fn part_1() -> Result<(), Error> {
    let mut tester = SledTester::new();

    let valid_entries = INPUT
        .iter()
        .map(|input| tester.test(input))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .filter(|is_valid| *is_valid)
        .count();

    println!("day_02::part_1: valid_entries = {}", valid_entries);

    Ok(())
}

fn part_2() -> Result<(), Error> {
    let mut tester = TobogganTester::new();

    let valid_entries = INPUT
        .iter()
        .map(|input| tester.test(input))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .filter(|is_valid| *is_valid)
        .count();

    println!("day_02::part_2: valid_entries = {}", valid_entries);

    Ok(())
}

pub fn is_valid_password_sled_rental(entry: &str) -> Result<bool, Error> {
    let mut policy_password_split = entry.split(':');

    let policy: PasswordPolicySledRental = policy_password_split
        .next()
        .ok_or(Error::MissingPolicy)?
        .parse()
        .map_err(Error::InvalidSledPolicy)?;

    let password = policy_password_split
        .next()
        .ok_or(Error::MissingPassword)?
        .trim();

    let valid = policy.is_valid_password(password);

    Ok(valid)
}

pub fn is_valid_password_toboggan_rental(entry: &str) -> Result<bool, Error> {
    let mut policy_password_split = entry.split(':');

    let policy: PasswordPolicyTobogganRental = policy_password_split
        .next()
        .ok_or(Error::MissingPolicy)?
        .parse()
        .map_err(Error::InvalidTobogganPolicy)?;

    let password = policy_password_split
        .next()
        .ok_or(Error::MissingPassword)?
        .trim();

    let valid = policy.is_valid_password(password);

    Ok(valid)
}

#[cfg(test)]
mod test {
    #[test]
    fn is_valid_password_sled_rental() {
        for (input, expected) in &super::TEST_INPUT_SLED_RENTAL {
            let got = super::is_valid_password_sled_rental(input);

            println!("input = {}", input);
            assert_eq!(*expected, got);
        }
    }

    #[test]
    fn is_valid_password_toboggan_rental() {
        for (input, expected) in &super::TEST_INPUT_TOBOGGAN_RENTAL {
            let got = super::is_valid_password_toboggan_rental(input);

            println!("input = {}", input);
            assert_eq!(*expected, got);
        }
    }

    #[test]
    fn part_1() {
        let expected = 591;
        let got = super::INPUT
            .iter()
            .filter_map(|input| super::is_valid_password_sled_rental(input).ok())
            .filter(|is_valid| *is_valid)
            .count();

        assert_eq!(expected, got);
    }

    #[test]
    fn part_2() {
        let expected = 335;
        let got = super::INPUT
            .iter()
            .filter_map(|input| super::is_valid_password_toboggan_rental(input).ok())
            .filter(|is_valid| *is_valid)
            .count();

        assert_eq!(expected, got);
    }
}

#[cfg(test)]
mod bench {
    use test::Bencher;

    #[bench]
    fn is_valid_password_sled_rental(b: &mut Bencher) {
        b.iter(|| {
            let _ = super::is_valid_password_sled_rental("1-3 a: abcde");
        });
    }

    #[bench]
    fn is_valid_password_toboggan_rental(b: &mut Bencher) {
        b.iter(|| {
            let _ = super::is_valid_password_toboggan_rental("1-3 a: abcde");
        });
    }

    #[bench]
    fn part_1(b: &mut Bencher) {
        let mut tester = super::TobogganTester::new();

        b.iter(|| {
            super::INPUT
                .iter()
                .filter_map(|input| tester.test(input).ok())
                .filter(|is_valid| *is_valid)
                .count();
        });
    }

    #[bench]
    fn part_2(b: &mut Bencher) {
        let mut tester = super::SledTester::new();

        b.iter(|| {
            super::INPUT
                .iter()
                .filter_map(|input| tester.test(input).ok())
                .into_iter()
                .filter(|is_valid| *is_valid)
                .count();
        });
    }
}
