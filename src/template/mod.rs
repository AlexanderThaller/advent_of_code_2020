use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("example error")]
    Example,
}

pub fn run() -> Result<(), Error> {
    part_1()?;
    part_2()?;

    Ok(())
}

pub fn part_1() -> Result<(), Error> {
    todo!()
}

pub fn part_2() -> Result<(), Error> {
    todo!()
}
