use thiserror::Error;

#[allow(clippy::empty_enum)]
#[derive(Debug, Error)]
pub enum Error {}

pub fn run() -> Result<(), Error> {
    println!("template::part_1: value = {}", part_1()?);
    println!("template::part_2: value = {}", part_2()?);

    Ok(())
}

pub fn part_1() -> Result<usize, Error> {
    Ok(0)
}

pub fn part_2() -> Result<usize, Error> {
    Ok(0)
}

#[cfg(test)]
mod test {
    #[test]
    fn part_1() {
        let expected = 0;
        let got = super::part_1().unwrap();

        assert_eq!(expected, got)
    }

    #[test]
    fn part_2() {
        let expected = 0;
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
