#![warn(clippy::pedantic)]
#![warn(clippy::unwrap_used)]
#![warn(rust_2018_idioms, unused_lifetimes, missing_debug_implementations)]
#![forbid(unsafe_code)]
#![feature(test)]
#[cfg(test)]
extern crate test;

mod day01;

use anyhow::Error;

fn main() -> Result<(), Error> {
    day01::run()?;

    Ok(())
}
