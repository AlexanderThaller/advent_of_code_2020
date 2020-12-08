use thiserror::Error;

mod handheld;

use handheld::{
    fixer::Fixer,
    Handheld,
};

#[derive(Debug, Error)]
pub enum Error {
    #[error("problem while parsing handheld: {0}")]
    HandheldParse(handheld::Error),

    #[error("problem while running handheld: {0}")]
    HandheldRun(handheld::Error),
}

pub fn run() -> Result<(), Error> {
    println!("template::part_1: value = {}", part_1()?);
    println!("template::part_2: value = {}", part_2()?);

    Ok(())
}

pub fn part_1() -> Result<isize, Error> {
    const INPUT: &str = include_str!("input.txt");

    let handlheld = INPUT.parse::<Handheld>().map_err(Error::HandheldParse)?;
    let acc = match handlheld.run() {
        Ok(h) | Err(handheld::Error::LoopFound(h)) => h.accumulator,
        Err(err) => return Err(Error::HandheldRun(err)),
    };

    Ok(acc)
}

pub fn part_2() -> Result<isize, Error> {
    const INPUT: &str = include_str!("input.txt");
    let handlheld = INPUT.parse::<Handheld>().map_err(Error::HandheldParse)?;
    let mut fixer = Fixer::from(handlheld);
    let acc = fixer.run().expect("step failure").accumulator;

    Ok(acc)
}

#[cfg(test)]
mod test {
    #[test]
    fn part_1() {
        let expected = 2080;
        let got = super::part_1().unwrap();

        assert_eq!(expected, got)
    }

    #[test]
    fn part_2() {
        let expected = 2477;
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
