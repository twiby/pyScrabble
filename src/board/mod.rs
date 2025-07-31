mod _board;
use _board::Board;

mod tile;
mod values;

pub use crate::constraints::PotentialWordConditionsBuilder;
pub use crate::constraints::WordToFill;

pub const SIDE: usize = 15;
pub const SIZE: usize = SIDE * SIDE;

#[derive(Debug, PartialEq)]
pub enum DeserializingError {
    UnknownSymbol(String),
    WrongLength(String),
}

#[derive(Debug, PartialEq)]
pub enum WordError {
    TileOccupied(String),
    UnexpectedUnderscore(String),
    UnknownChar(String),
    UnknownConstraint(String),
}

pub mod transposition {
    pub struct Transposed;
    pub struct NotTransposed;
    pub trait TransposedState {
        fn transposed_coord(x: usize, y: usize) -> (usize, usize);
    }
    impl TransposedState for Transposed {
        fn transposed_coord(x: usize, y: usize) -> (usize, usize) {
            (y, x)
        }
    }
    impl TransposedState for NotTransposed {
        fn transposed_coord(x: usize, y: usize) -> (usize, usize) {
            (x, y)
        }
    }
}

pub trait BoardService {
    #[allow(unused)]
    fn serialize<T: transposition::TransposedState>(&self) -> String;
    fn deserialize(message: &str) -> Result<Board, DeserializingError>;
    fn get_conditions<T: transposition::TransposedState, PWCB>(
        &self,
        x: usize,
        y: usize,
        conditions: &mut PWCB,
    ) where
        PWCB: PotentialWordConditionsBuilder;
    fn get_score<T: transposition::TransposedState>(
        &self,
        word: &[char],
        x: usize,
        y: usize,
    ) -> Result<usize, WordError>;
}

pub fn deserialize(message: &str) -> Result<Board, DeserializingError> {
    Board::deserialize(message)
}
