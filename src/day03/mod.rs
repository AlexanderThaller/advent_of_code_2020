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

pub fn part_1() -> Result<(), Error> {
    let input = include_str!("input.txt").parse()?;
    let mut rider = rider::new(input);
    rider.ride();

    println!("day_01::part_1: trees_seen = {}", rider.trees_seen);

    Ok(())
}

pub fn part_2() -> Result<(), Error> {
    todo!()
}
