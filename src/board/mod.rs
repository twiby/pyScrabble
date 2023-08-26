mod board;
use board::Board;

mod values;
mod tile;

pub use crate::constraints::WordToFill;
pub use crate::constraints::PotentialWordConditionsBuilder;

pub const SIDE: usize = 15;
pub const SIZE: usize = SIDE * SIDE;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum DeserializingError {
	UnknownSymbol(String),
	WrongLength(String)
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum WordError {
	TileOccupied(String),
	UnexpectedUnderscore(String),
	UnknownChar(String),
	UnknownConstraint(String)
} 

pub mod transposition
{
	pub struct Transposed;
	pub struct NotTransposed;
	pub trait TransposedState {
		fn transposed_coord(x: usize, y: usize) -> (usize, usize);
	}
	impl TransposedState for Transposed {
		fn transposed_coord(x: usize, y: usize) -> (usize, usize) { (y,x) }
	}
	impl TransposedState for NotTransposed {
		fn transposed_coord(x: usize, y: usize) -> (usize, usize) { (x,y) }
	}
}

pub trait BoardService {
	fn serialize<T: transposition::TransposedState>(&self) -> String;
	fn deserialize(message: &str) -> Result<Board, DeserializingError>;
	fn get_conditions<T: transposition::TransposedState, PWCB>(&self, x: usize, y: usize, conditions: &mut PWCB)
	where PWCB: PotentialWordConditionsBuilder;
	fn get_score<T: transposition::TransposedState>(&self, word: &[char], x: usize, y: usize) -> Result<usize, WordError>;
}

pub fn deserialize(message: &str) -> Result<Board, DeserializingError> {
	return Board::deserialize(message.clone());
}
