pub(super) mod sled_rental {
    use thiserror::Error;

    #[derive(Debug, Error, Eq, PartialEq)]
    pub enum Error {
        #[error("missing minmax in input")]
        MissingMinMax,

        #[error("missing character in input")]
        MissingCharacter,

        #[error("missing min in input")]
        MissingMin,

        #[error("invalid min in input: {0}")]
        InvalidMin(std::num::ParseIntError),

        #[error("missing max in input")]
        MissingMax,

        #[error("invalid max in input: {0}")]
        InvalidMax(std::num::ParseIntError),
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
            let mut minmax_character_split = s.split(' ');

            let mut minmax = minmax_character_split
                .next()
                .ok_or(Error::MissingMinMax)?
                .split('-');

            let character = minmax_character_split
                .next()
                .ok_or(Error::MissingCharacter)?
                .chars()
                .next()
                .ok_or(Error::MissingCharacter)?;

            let min = minmax
                .next()
                .ok_or(Error::MissingMin)?
                .parse()
                .map_err(Error::InvalidMin)?;

            let max = minmax
                .next()
                .ok_or(Error::MissingMax)?
                .parse()
                .map_err(Error::InvalidMax)?;

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

            let expected = Ok(super::PasswordPolicy {
                min: 1,
                max: 3,
                character: 'a',
            });

            let got = super::PasswordPolicy::from_str(INPUT);

            assert_eq!(expected, got);
        }
    }
}

pub(super) mod toboggan_rental {
    use thiserror::Error;

    #[derive(Debug, Error, Eq, PartialEq)]
    pub enum Error {
        #[error("missing index in input")]
        MissingIndex,

        #[error("missing character in input")]
        MissingCharacter,

        #[error("missing contains index in input")]
        MissingContainsIndex,

        #[error("invalid contains index in input: {0}")]
        InvalidContainsIndex(std::num::ParseIntError),

        #[error("missing not contains index in input")]
        MissingNotContainsIndex,

        #[error("invalid not contains index in input: {0}")]
        InvalidNotContainsIndex(std::num::ParseIntError),
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
            let mut index_character_split = s.split(' ');
            let mut index = index_character_split
                .next()
                .ok_or(Error::MissingIndex)?
                .split('-');

            let character = index_character_split
                .next()
                .ok_or(Error::MissingCharacter)?
                .chars()
                .next()
                .ok_or(Error::MissingCharacter)?;

            let contains_index = index
                .next()
                .ok_or(Error::MissingContainsIndex)?
                .parse::<usize>()
                .map_err(Error::InvalidContainsIndex)?
                - 1;

            let not_contains_index = index
                .next()
                .ok_or(Error::MissingNotContainsIndex)?
                .parse::<usize>()
                .map_err(Error::InvalidNotContainsIndex)?
                - 1;

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
