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
    use eye_color::EyeColor;
    use hair_color::HairColor;
    use height::Height;
    use itertools::Itertools;
    use passport_id::PassportID;
    use thiserror::Error;

    #[derive(Debug, Error, Eq, PartialEq)]
    pub enum Error {
        #[error("missing field {0:?}")]
        MissingField(&'static str),
    }

    #[derive(Debug, Eq, PartialEq)]
    pub struct Passport {
        //(Birth Year)
        byr: usize,
        //(Issue Year)
        iyr: usize,
        //(Expiration Year)
        eyr: usize,
        //(Height)
        hgt: Height,
        //(Hair Color)
        hcl: HairColor,
        //(Eye Color)
        ecl: EyeColor,
        //(Passport ID)
        pid: PassportID,
        //(Country ID)
        cid: Option<usize>,
    }

    impl std::str::FromStr for Passport {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            #[derive(Debug, Default)]
            struct Builder {
                byr: Option<usize>,
                iyr: Option<usize>,
                eyr: Option<usize>,
                hgt: Option<Height>,
                hcl: Option<HairColor>,
                ecl: Option<EyeColor>,
                pid: Option<PassportID>,
                cid: Option<usize>,
            };

            let values = s
                .split_whitespace()
                .map(|entry| entry.splitn(2, ':').collect_tuple().unwrap())
                .collect::<Vec<_>>();

            let mut builder = Builder::default();
            for (key, value) in values {
                match key {
                    "byr" => builder.byr = value.parse().ok().filter(|v| 1920 <= *v && *v <= 2002),
                    "iyr" => builder.iyr = value.parse().ok().filter(|v| 2010 <= *v && *v <= 2020),
                    "eyr" => builder.eyr = value.parse().ok().filter(|v| 2020 <= *v && *v <= 2030),
                    "hgt" => {
                        builder.hgt = value.parse().ok().filter(|v| match v {
                            Height::Centimeters(h) => 150 <= *h && *h <= 193,
                            Height::Inches(h) => 59 <= *h && *h <= 76,
                        })
                    }
                    "hcl" => builder.hcl = value.parse().ok(),
                    "ecl" => builder.ecl = value.parse().ok(),
                    "pid" => builder.pid = value.parse().ok(),
                    "cid" => builder.cid = value.parse().ok(),

                    _ => panic!("unkown key {}", key),
                }
            }

            Ok(Passport {
                byr: builder.byr.ok_or(Error::MissingField("byr"))?,
                iyr: builder.iyr.ok_or(Error::MissingField("iyr"))?,
                eyr: builder.eyr.ok_or(Error::MissingField("eyr"))?,
                hgt: builder.hgt.ok_or(Error::MissingField("hgt"))?,
                hcl: builder.hcl.ok_or(Error::MissingField("hcl"))?,
                ecl: builder.ecl.ok_or(Error::MissingField("ecl"))?,
                pid: builder.pid.ok_or(Error::MissingField("pid"))?,
                cid: builder.cid,
            })
        }
    }

    #[cfg(test)]
    mod test {
        use super::{
            eye_color::EyeColor,
            hair_color::HairColor,
            height::Height,
            passport_id::PassportID,
            Error,
            Passport,
        };

        mod from_str {
            use super::{
                Error,
                EyeColor,
                HairColor,
                Height,
                Passport,
                PassportID,
            };

            #[test]
            fn part_1_example1() {
                const INPUT: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 \
                                     cid:147 hgt:183cm";

                let expected = Passport {
                    ecl: EyeColor::Gry,
                    pid: PassportID {
                        value: [8, 6, 0, 0, 3, 3, 3, 2, 7],
                    },
                    eyr: 2020,
                    hcl: HairColor {
                        value: "fffffd".into(),
                    },
                    byr: 1937,
                    iyr: 2017,
                    cid: Some(147),
                    hgt: Height::Centimeters(183),
                };

                let got = INPUT.parse().expect("invalid input");

                assert_eq!(expected, got)
            }

            #[test]
            fn part_1_example2() {
                const INPUT: &str =
                    "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929";

                let expected: Result<Passport, Error> = Err(Error::MissingField("hgt"));
                let got = INPUT.parse();

                assert_eq!(expected, got)
            }

            #[test]
            fn part_1_example3() {
                const INPUT: &str =
                    "hcl:#ae17e1 iyr:2013 eyr:2024 ecl:brn pid:760753108 byr:1931 hgt:179cm";

                let expected = Passport {
                    ecl: EyeColor::Brn,
                    pid: PassportID {
                        value: [7, 6, 0, 7, 5, 3, 1, 0, 8],
                    },
                    eyr: 2024,
                    hcl: HairColor {
                        value: "ae17e1".into(),
                    },
                    byr: 1931,
                    iyr: 2013,
                    cid: None,
                    hgt: Height::Centimeters(179),
                };
                let got = INPUT.parse().expect("invalid input");

                assert_eq!(expected, got)
            }

            #[test]
            fn part_1_example4() {
                const INPUT: &str = "hcl:#cfa07d eyr:2025 pid:166559648 iyr:2011 ecl:brn hgt:59in";

                let expected: Result<Passport, Error> = Err(Error::MissingField("byr"));
                let got = INPUT.parse();

                assert_eq!(expected, got)
            }

            #[test]
            fn part_2_invalid_example1() {
                const INPUT: &str =
                    "eyr:1972 cid:100 hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926";

                let expected: Result<Passport, Error> = Err(Error::MissingField("eyr"));
                let got = INPUT.parse();

                assert_eq!(expected, got)
            }

            #[test]
            fn part_2_invalid_example2() {
                const INPUT: &str =
                    "iyr:2019 hcl:#602927 eyr:1967 hgt:170cm ecl:grn pid:012533040 byr:1946";

                let expected: Result<Passport, Error> = Err(Error::MissingField("eyr"));
                let got = INPUT.parse();

                assert_eq!(expected, got)
            }

            #[test]
            fn part_2_invalid_example3() {
                const INPUT: &str =
                    "hcl:dab227 iyr:2012 ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277";

                let expected: Result<Passport, Error> = Err(Error::MissingField("hcl"));
                let got = INPUT.parse();

                assert_eq!(expected, got)
            }

            #[test]
            fn part_2_invalid_example4() {
                const INPUT: &str =
                    "hgt:59cm ecl:zzz eyr:2038 hcl:74454a iyr:2023 pid:3556412378 byr:2007";

                let expected: Result<Passport, Error> = Err(Error::MissingField("byr"));
                let got = INPUT.parse();

                assert_eq!(expected, got)
            }

            #[test]
            fn part_2_valid_example1() {
                const INPUT: &str =
                    "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980 hcl:#623a2f";

                let expected = Passport {
                    pid: PassportID {
                        value: [0, 8, 7, 4, 9, 9, 7, 0, 4],
                    },
                    hgt: Height::Inches(74),
                    ecl: EyeColor::Grn,
                    iyr: 2012,
                    eyr: 2030,
                    byr: 1980,
                    hcl: HairColor {
                        value: "623a2f".into(),
                    },
                    cid: None,
                };

                let got = INPUT.parse().expect("invalid input");

                assert_eq!(expected, got)
            }

            #[test]
            fn part_2_valid_example2() {
                const INPUT: &str = "eyr:2029 ecl:blu cid:129 byr:1989 iyr:2014 pid:896056539 \
                                     hcl:#a97842 hgt:165cm";

                let expected = Passport {
                    eyr: 2029,
                    ecl: EyeColor::Blu,
                    cid: Some(129),
                    byr: 1989,
                    iyr: 2014,
                    pid: PassportID {
                        value: [8, 9, 6, 0, 5, 6, 5, 3, 9],
                    },
                    hcl: HairColor {
                        value: "a97842".into(),
                    },
                    hgt: Height::Centimeters(165),
                };

                let got = INPUT.parse().expect("invalid input");

                assert_eq!(expected, got)
            }

            #[test]
            fn part_2_valid_example3() {
                const INPUT: &str =
                    "hcl:#888785 hgt:164cm byr:2001 iyr:2015 cid:88 pid:545766238 ecl:hzl eyr:2022";

                let expected = Passport {
                    hcl: HairColor {
                        value: "888785".into(),
                    },
                    hgt: Height::Centimeters(164),
                    byr: 2001,
                    iyr: 2015,
                    cid: Some(88),
                    pid: PassportID {
                        value: [5, 4, 5, 7, 6, 6, 2, 3, 8],
                    },
                    ecl: EyeColor::Hzl,
                    eyr: 2022,
                };

                let got = INPUT.parse().expect("invalid input");

                assert_eq!(expected, got)
            }
        }
    }

    mod height {
        use thiserror::Error;

        #[derive(Debug, Error)]
        pub enum Error {
            #[error("invalid input")]
            InvalidInput,
        }

        #[derive(Debug, Eq, PartialEq)]
        pub enum Height {
            Inches(usize),
            Centimeters(usize),
        }

        impl std::str::FromStr for Height {
            type Err = Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let chars = s.chars().collect::<Vec<_>>();
                let out = match chars.as_slice() {
                    [x @ .., 'c', 'm'] => {
                        Height::Centimeters(x.iter().cloned().collect::<String>().parse().unwrap())
                    }
                    [x @ .., 'i', 'n'] => {
                        Height::Inches(x.iter().cloned().collect::<String>().parse().unwrap())
                    }
                    _ => return Err(Error::InvalidInput),
                };

                Ok(out)
            }
        }
    }

    mod hair_color {
        use thiserror::Error;

        #[derive(Debug, Error)]
        pub enum Error {
            #[error("input does not start with #")]
            NotStartWithHash,

            #[error("input color contains invalid character")]
            InvalidCharacter,

            #[error("input color value is too short")]
            TooShort,
        }

        #[derive(Debug, Eq, PartialEq)]
        pub struct HairColor {
            pub(super) value: String,
        }

        impl std::str::FromStr for HairColor {
            type Err = Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                if !s.starts_with('#') {
                    return Err(Error::NotStartWithHash);
                }

                const VALID_CHARS: [char; 17] = [
                    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e',
                    'f',
                ];

                let value = s
                    .chars()
                    .skip(1)
                    .map(|c| {
                        if !VALID_CHARS.contains(&c) {
                            Err(Error::InvalidCharacter)
                        } else {
                            Ok(c)
                        }
                    })
                    .collect::<Result<String, _>>()?;

                if value.len() != 6 {
                    return Err(Error::TooShort);
                }

                Ok(Self { value })
            }
        }
    }

    mod eye_color {
        use thiserror::Error;

        #[derive(Debug, Error)]
        pub enum Error {
            #[error("invalid eye color {0:?}")]
            InvalidEyeColor(String),
        }

        #[derive(Debug, Eq, PartialEq)]
        pub enum EyeColor {
            Amb,
            Blu,
            Brn,
            Gry,
            Grn,
            Hzl,
            Oth,
        }

        impl std::str::FromStr for EyeColor {
            type Err = Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    "amb" => Ok(Self::Amb),
                    "blu" => Ok(Self::Blu),
                    "brn" => Ok(Self::Brn),
                    "gry" => Ok(Self::Gry),
                    "grn" => Ok(Self::Grn),
                    "hzl" => Ok(Self::Hzl),
                    "oth" => Ok(Self::Oth),
                    _ => Err(Error::InvalidEyeColor(s.into())),
                }
            }
        }
    }

    mod passport_id {
        use std::convert::TryInto;
        use thiserror::Error;

        #[derive(Debug, Error)]
        pub enum Error {
            #[error("passport id is too short")]
            TooShort,

            #[error("passport contains invalid digit")]
            InvalidDigit(std::num::ParseIntError),
        }

        #[derive(Debug, Eq, PartialEq)]
        pub struct PassportID {
            pub(super) value: [usize; 9],
        }

        impl std::str::FromStr for PassportID {
            type Err = Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let chars = s.chars().collect::<Vec<_>>();
                if chars.len() != 9 {
                    return Err(Error::TooShort);
                }

                let value = chars
                    .into_iter()
                    .map(|c| c.to_string().parse().map_err(Error::InvalidDigit))
                    .collect::<Result<Vec<usize>, _>>()?
                    .try_into()
                    .unwrap();

                Ok(Self { value })
            }
        }
    }
}
