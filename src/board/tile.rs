#[derive(Copy, Clone, Debug)]
pub enum PlayedTile {
    Letter(char),
    Joker(char),
}
#[derive(Copy, Clone, Debug)]
pub enum BoardTile {
    Empty,
    LetterBonus(u8),
    WordBonus(u8),
}

#[derive(Copy, Clone, Debug)]
pub enum Tile {
    Played(PlayedTile),
    Board(BoardTile),
}

impl Tile {
    pub fn is_occupied(&self) -> bool {
        self.letter().is_some()
    }

    pub fn letter(&self) -> Option<char> {
        match self {
            Tile::Played(PlayedTile::Letter(c)) => Some(*c),
            Tile::Played(PlayedTile::Joker(c)) => Some(c.to_ascii_uppercase()),
            _ => None,
        }
    }
}
