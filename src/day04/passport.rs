use thiserror::Error;

use passport::Passport;

#[derive(Debug, Error, Eq, PartialEq)]
pub enum Error {}

#[derive(Debug, Eq, PartialEq, Default)]
pub struct Passports {
    entries: Vec<Passport>,
}

impl std::str::FromStr for Passports {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let entries = s
            .split("\n\n")
            .filter_map(|line| line.parse().ok())
            .collect::<Vec<_>>();

        Ok(Self { entries })
    }
}

impl Passports {
    pub fn len(&self) -> usize {
        self.entries.len()
    }
}

#[cfg(test)]
mod test {
    use super::Passports;

    mod from_str {
        use super::Passports;

        #[test]
        fn input() {
            const INPUT: &str = include_str!("input_test.txt");
            let expected = 2;
            let got = INPUT.parse::<Passports>().unwrap().len();

            assert_eq!(expected, got);
        }
    }
}

mod passport {
    use itertools::Itertools;
    use thiserror::Error;

    #[derive(Debug, Error, Eq, PartialEq)]
    pub enum Error {
        #[error("missing field {0:?}")]
        MissingField(&'static str),
    }

    #[derive(Debug, Eq, PartialEq)]
    pub struct Passport {
        //(Birth Year)
        byr: String,
        //(Issue Year)
        iyr: String,
        //(Expiration Year)
        eyr: String,
        //(Height)
        hgt: String,
        //(Hair Color)
        hcl: String,
        //(Eye Color)
        ecl: String,
        //(Passport ID)
        pid: String,
        //(Country ID)
        cid: Option<String>,
    }

    impl std::str::FromStr for Passport {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            #[derive(Debug, Default)]
            struct Builder<'a> {
                byr: Option<&'a str>,
                iyr: Option<&'a str>,
                eyr: Option<&'a str>,
                hgt: Option<&'a str>,
                hcl: Option<&'a str>,
                ecl: Option<&'a str>,
                pid: Option<&'a str>,
                cid: Option<&'a str>,
            };

            let values = s
                .split_whitespace()
                .map(|entry| entry.splitn(2, ':').collect_tuple().unwrap())
                .collect::<Vec<_>>();

            let mut builder = Builder::default();
            for (key, value) in values {
                match key {
                    "byr" => builder.byr = Some(value),
                    "iyr" => builder.iyr = Some(value),
                    "eyr" => builder.eyr = Some(value),
                    "hgt" => builder.hgt = Some(value),
                    "hcl" => builder.hcl = Some(value),
                    "ecl" => builder.ecl = Some(value),
                    "pid" => builder.pid = Some(value),
                    "cid" => builder.cid = Some(value),

                    _ => panic!("unkown key {}", key),
                }
            }

            Ok(Passport {
                byr: builder.byr.ok_or(Error::MissingField("byr"))?.to_string(),
                iyr: builder.iyr.ok_or(Error::MissingField("iyr"))?.to_string(),
                eyr: builder.eyr.ok_or(Error::MissingField("eyr"))?.to_string(),
                hgt: builder.hgt.ok_or(Error::MissingField("hgt"))?.to_string(),
                hcl: builder.hcl.ok_or(Error::MissingField("hcl"))?.to_string(),
                ecl: builder.ecl.ok_or(Error::MissingField("ecl"))?.to_string(),
                pid: builder.pid.ok_or(Error::MissingField("pid"))?.to_string(),
                cid: builder.cid.map(ToString::to_string),
            })
        }
    }

    #[cfg(test)]
    mod test {
        use super::{
            Error,
            Passport,
        };

        mod from_str {
            use super::{
                Error,
                Passport,
            };

            #[test]
            fn part_1_example1() {
                const INPUT: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm";

                let expected = Passport {
                    ecl: "gry".into(),
                    pid: "860033327".into(),
                    eyr: "2020".into(),
                    hcl: "#fffffd".into(),
                    byr: "1937".into(),
                    iyr: "2017".into(),
                    cid: Some("147".into()),
                    hgt: "183cm".into(),
                };

                let got = INPUT.parse().expect("invalid input");

                assert_eq!(expected, got)
            }

            #[test]
            fn part_1_example2() {
                const INPUT: &str = "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929";

                let expected: Result<Passport, Error> = Err(Error::MissingField("hgt"));
                let got = INPUT.parse();

                assert_eq!(expected, got)
            }

            #[test]
            fn part_1_example3() {
                const INPUT: &str = "hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm";

                let expected = Passport {
                    ecl: "brn".into(),
                    pid: "760753108".into(),
                    eyr: "2024".into(),
                    hcl: "#ae17e1".into(),
                    byr: "1931".into(),
                    iyr: "2013".into(),
                    cid: None,
                    hgt: "179cm".into(),
                };
                let got = INPUT.parse().expect("invalid input");

                assert_eq!(expected, got)
            }

            #[test]
            fn part_1_example4() {
                const INPUT: &str = "hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

                let expected: Result<Passport, Error> = Err(Error::MissingField("byr"));
                let got = INPUT.parse();

                assert_eq!(expected, got)
            }
        }
    }
}
