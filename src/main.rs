#![feature(iterator_fold_self)]
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
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

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
            "day05" => day05::run()?,
            "day06" => day06::run()?,
            "day07" => day07::run()?,
            "day08" => day08::run()?,
            "day09" => day09::run()?,
            _ => return Err(anyhow!("unkown day {}", day)),
        }
    } else {
        day01::run()?;
        day02::run()?;
        day03::run()?;
        day04::run()?;
        day05::run()?;
        day06::run()?;
        day07::run()?;
        day08::run()?;
        day09::run()?;
    }

    Ok(())
}
