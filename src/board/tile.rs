#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
pub enum PlayedTile {
	LetterTile(char),
	JokerTile(char)
}
#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
pub enum BoardTile {
	EmptyTile,
	LetterBonusTile(u8),
	WordBonusTile(u8)
}

#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
pub enum Tile{
	Played(PlayedTile),
	Board(BoardTile)
}

impl Tile {
	pub fn is_occupied(&self) -> bool {
		match self.letter() {
			None => false,
			Some(_) => true
		}
	}

	pub fn letter(&self) -> Option<char> {
		match self {
			Tile::Played(PlayedTile::LetterTile(c)) => Some(*c),
			Tile::Played(PlayedTile::JokerTile(c)) => Some(c.to_ascii_uppercase()),
			_ => None
		}
	}
}