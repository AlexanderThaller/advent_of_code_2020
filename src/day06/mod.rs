use thiserror::Error;

mod answers;

#[allow(clippy::empty_enum)]
#[derive(Debug, Error)]
pub enum Error {}

pub fn run() -> Result<(), Error> {
    println!("day_06::part_1: answers with yes = {}", part_1()?);
    println!("day_06::part_2: value = {:?}", part_2()?);

    Ok(())
}

pub fn part_1() -> Result<usize, Error> {
    let count = answers::count_yes(include_str!("input.txt"));

    Ok(count)
}

pub fn part_2() -> Result<usize, Error> {
    let count = answers::count_yes_all(include_str!("input.txt"));

    Ok(count)
}

#[cfg(test)]
mod test {
    #[test]
    fn part_1() {
        let expected = 6590;
        let got = super::part_1().unwrap();

        assert_eq!(expected, got)
    }

    #[test]
    fn part_2() {
        let expected = 3288;
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
        })
    }

    #[bench]
    fn part_2(b: &mut Bencher) {
        b.iter(|| {
            let _ = super::part_2();
        })
    }
}
