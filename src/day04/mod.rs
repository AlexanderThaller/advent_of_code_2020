use thiserror::Error;

mod passport;

use passport::Passports;

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
    let valid_passports = INPUT.parse::<Passports>().unwrap().len();

    Ok(valid_passports)
}
