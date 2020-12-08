use thiserror::Error;

mod bag;

use bag::Bags;

#[allow(clippy::empty_enum)]
#[derive(Debug, Error)]
pub enum Error {}

pub fn run() -> Result<(), Error> {
    println!("template::part_1: contain shiny gold bag = {}", part_1()?);
    println!(
        "template::part_2: need to be in shiny gold bag = {}",
        part_2()?
    );

    Ok(())
}

pub fn part_1() -> Result<usize, Error> {
    let count = Bags::from(include_str!("input.txt"))
        .find_all_containers("shiny gold")
        .len();

    Ok(count)
}

pub fn part_2() -> Result<usize, Error> {
    let count = Bags::from(include_str!("input.txt"))
        .must_contain("shiny gold")
        .into_iter()
        .map(|(_, count)| count)
        .sum::<usize>();

    Ok(count)
}

#[cfg(test)]
mod test {
    #[test]
    fn part_1() {
        let expected = 119;
        let got = super::part_1().unwrap();

        assert_eq!(expected, got)
    }

    #[test]
    fn part_2() {
        let expected = 155802;
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
