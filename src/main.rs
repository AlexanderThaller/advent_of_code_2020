#![warn(clippy::pedantic)]
#![warn(clippy::unwrap_used)]
#![warn(rust_2018_idioms, unused_lifetimes, missing_debug_implementations)]
#![forbid(unsafe_code)]
#![feature(test)]
#[cfg(test)]
extern crate test;

#[allow(dead_code)]
mod template;

mod day01;
mod day02;
mod day03;

use anyhow::Error;

fn main() -> Result<(), Error> {
    day01::run()?;
    day02::run()?;
    day03::run()?;

    Ok(())
}
