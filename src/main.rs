#![warn(clippy::pedantic)]
//#![warn(clippy::unwrap_used)]
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
mod day04;

use anyhow::{
    anyhow,
    Error,
};

fn main() -> Result<(), Error> {
    if let Some(day) = std::env::args().nth(1) {
        match day.as_str() {
            "day01" => day01::run()?,
            "day02" => day02::run()?,
            "day03" => day03::run()?,
            "day04" => day04::run()?,
            _ => return Err(anyhow!("unkown day {}", day)),
        }
    } else {
        day01::run()?;
        day02::run()?;
        day03::run()?;
        day04::run()?;
    }

    Ok(())
}
