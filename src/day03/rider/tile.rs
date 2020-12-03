use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("don't know how to parse {0:?}")]
    UnkownTile(char),
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
pub enum Tile {
    Tree,
    Air,
}

impl std::convert::TryFrom<char> for Tile {
    type Error = Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '#' => Ok(Self::Tree),
            '.' => Ok(Self::Air),
            _ => Err(Error::UnkownTile(c)),
        }
    }
}
