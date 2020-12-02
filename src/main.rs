#![feature(test)]
#[cfg(test)]
extern crate test;

mod day01;

use anyhow::Error;

fn main() -> Result<(), Error> {
    day01::run()?;

    Ok(())
}
