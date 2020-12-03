pub(super) mod sled_rental {
    use scan_fmt::scan_fmt;
    use thiserror::Error;

    #[derive(Debug, Error, Eq, PartialEq)]
    pub enum Error {
        #[error("invalid input for policy: {0}")]
        InvalidPolicy(String),
    }

    #[derive(Debug, Eq, PartialEq)]
    pub struct PasswordPolicy {
        min: usize,
        max: usize,
        character: char,
    }

    impl PasswordPolicy {
        pub fn is_valid_password(&self, password: &str) -> bool {
            let occurences = password.chars().filter(|ch| *ch == self.character).count();

            if occurences < self.min {
                return false;
            }

            if occurences > self.max {
                return false;
            }

            true
        }
    }

    impl std::str::FromStr for PasswordPolicy {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (min, max, character) = scan_fmt!(s, "{}-{} {}", usize, usize, char)
                .map_err(|e| Error::InvalidPolicy(format!("{}", e)))?;

            Ok(Self {
                min,
                max,
                character,
            })
        }
    }

    #[cfg(test)]
    mod test {
        use std::str::FromStr;

        #[test]
        fn from_str() {
            const INPUT: &str = "1-3 a";

            let expected = super::PasswordPolicy {
                min: 1,
                max: 3,
                character: 'a',
            };

            let got = super::PasswordPolicy::from_str(INPUT).expect("bad input");

            assert_eq!(expected, got);
        }
    }
}

pub(super) mod toboggan_rental {
    use scan_fmt::scan_fmt;
    use thiserror::Error;

    #[derive(Debug, Error, Eq, PartialEq)]
    pub enum Error {
        #[error("invalid input for policy: {0}")]
        InvalidPolicy(String),
    }

    #[derive(Debug, Eq, PartialEq)]
    pub struct PasswordPolicy {
        contains_index: usize,
        not_contains_index: usize,
        character: char,
    }

    impl PasswordPolicy {
        pub fn is_valid_password(&self, password: &str) -> bool {
            let chars = password.chars().collect::<Vec<_>>();

            let contains_char = chars[self.contains_index];
            let not_contains_char = chars[self.not_contains_index];

            let contains = contains_char == self.character;
            let not_contains = not_contains_char != self.character;

            if contains && not_contains {
                return true;
            }

            if !contains && !not_contains {
                return true;
            }

            false
        }
    }

    impl std::str::FromStr for PasswordPolicy {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let (contains_index, not_contains_index, character) =
                scan_fmt!(s, "{}-{} {}", usize, usize, char)
                    .map_err(|e| Error::InvalidPolicy(format!("{}", e)))?;

            // Make values one smaller so we have a zero indexed value for the index
            let contains_index = contains_index - 1;
            let not_contains_index = not_contains_index - 1;

            Ok(Self {
                contains_index,
                not_contains_index,
                character,
            })
        }
    }

    #[cfg(test)]
    mod test {
        use std::str::FromStr;

        #[test]
        fn from_str() {
            const INPUT: &str = "1-3 a";

            let expected = Ok(super::PasswordPolicy {
                contains_index: 0,
                not_contains_index: 2,
                character: 'a',
            });

            let got = super::PasswordPolicy::from_str(INPUT);

            assert_eq!(expected, got);
        }
    }
}
