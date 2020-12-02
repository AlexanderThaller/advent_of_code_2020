use itertools::Itertools;
use thiserror::Error;

const WANTED_SUM: usize = 2020;

#[cfg(test)]
const TEST_INPUT: [usize; 6] = [1721, 979, 366, 299, 675, 1456];

const INPUT_FIRST_PART: [usize; 200] = [
    1753, 1976, 1574, 308, 1384, 1191, 1731, 1829, 1658, 1908, 1663, 2001, 1298, 1888, 1134, 1213,
    965, 2009, 1071, 1591, 1402, 1184, 1836, 1536, 1038, 1871, 1354, 1149, 1863, 1728, 1896, 1599,
    1556, 1222, 1909, 1858, 1754, 1947, 1907, 1656, 1135, 1845, 1504, 1473, 1401, 1700, 1067, 1790,
    1783, 1539, 1087, 1614, 1856, 1895, 1564, 1106, 1204, 1492, 1361, 1897, 1977, 1210, 1867, 1797,
    1232, 1148, 1520, 1989, 210, 1259, 570, 1512, 1894, 1309, 1154, 1327, 1817, 1875, 1702, 1885,
    1664, 1220, 1208, 2000, 1178, 1423, 1454, 1780, 1710, 1362, 1816, 1491, 1363, 1478, 1648, 1163,
    1554, 1195, 1500, 1320, 1698, 1636, 1097, 1573, 1846, 1747, 1138, 1083, 1505, 1387, 1900, 1143,
    1905, 1826, 1735, 1496, 1687, 1704, 1916, 1991, 1750, 1637, 1742, 691, 1967, 1272, 1657, 1140,
    1070, 1985, 1405, 1959, 1218, 1878, 1340, 1722, 2003, 1258, 1726, 1766, 1868, 1714, 1463, 2006,
    1537, 1570, 1526, 1578, 1744, 1734, 1325, 196, 1935, 1849, 1424, 1972, 1602, 1859, 1341, 1177,
    1901, 1902, 1247, 2004, 1350, 1965, 1407, 836, 1899, 1804, 975, 1510, 1898, 1560, 1777, 1523,
    1822, 1830, 1855, 1839, 1482, 1661, 1835, 1343, 1278, 1449, 1136, 1732, 2008, 1686, 1775, 1952,
    1444, 1499, 1680, 1752, 1597, 1963, 1117, 776,
];

#[derive(Debug, Error)]
pub enum Error {
    #[error("did not find solution for day01::part_1")]
    Part1,

    #[error("did not find solution for day01::part_2")]
    Part2,
}

pub fn run() -> Result<(), Error> {
    part_1()?;
    part_2()?;

    Ok(())
}

fn part_1() -> Result<(), Error> {
    let (first, second) = find_two_entries(&INPUT_FIRST_PART).ok_or(Error::Part1)?;

    println!(
        "day_01::part_1: {} + {} = {}",
        first,
        second,
        first * second
    );

    Ok(())
}

fn part_2() -> Result<(), Error> {
    let (first, second, third) = find_three_entries(&INPUT_FIRST_PART).ok_or(Error::Part2)?;

    println!(
        "day_01::part_2: {} + {} + {} = {}",
        first,
        second,
        third,
        first * second * third
    );

    Ok(())
}

fn find_two_entries(entries: &[usize]) -> Option<(&usize, &usize)> {
    entries
        .iter()
        .tuple_combinations()
        .find(|(first, second)| *first + *second == WANTED_SUM)
}

fn find_three_entries(entries: &[usize]) -> Option<(&usize, &usize, &usize)> {
    entries
        .iter()
        .tuple_combinations()
        .find(|(first, second, third)| *first + *second + *third == WANTED_SUM)
}

#[cfg(test)]
mod test {
    #[test]
    fn find_two_entries() {
        let expected = Some((&1721, &299));
        let got = super::find_two_entries(&super::TEST_INPUT);

        assert_eq!(expected, got);
    }

    #[test]
    fn find_three_entries() {
        let expected = Some((&979, &366, &675));
        let got = super::find_three_entries(&super::TEST_INPUT);

        assert_eq!(expected, got);
    }

    #[test]
    fn part_1() {
        let expected = Some((&1184, &836));
        let got = super::find_two_entries(&super::INPUT_FIRST_PART);

        assert_eq!(expected, got);
    }

    #[test]
    fn part_2() {
        let expected = Some((&1614, &210, &196));
        let got = super::find_three_entries(&super::INPUT_FIRST_PART);

        assert_eq!(expected, got);
    }
}

#[cfg(test)]
mod bench {
    use test::Bencher;

    #[bench]
    fn find_two_entries(b: &mut Bencher) {
        b.iter(|| {
            super::find_two_entries(&super::TEST_INPUT);
        });
    }

    #[bench]
    fn find_three_entries(b: &mut Bencher) {
        b.iter(|| {
            super::find_three_entries(&super::TEST_INPUT);
        });
    }

    #[bench]
    fn part_1(b: &mut Bencher) {
        b.iter(|| {
            super::find_two_entries(&super::INPUT_FIRST_PART);
        });
    }

    #[bench]
    fn part_2(b: &mut Bencher) {
        b.iter(|| {
            super::find_three_entries(&super::INPUT_FIRST_PART);
        });
    }
}
