mod chess;

pub use chess::{Piece, Position as ChessPosition};

pub struct Queen(ChessPosition);

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self(position)
    }
}

impl Piece for Queen {
    fn can_attack<T: Piece>(&self, other: &T) -> bool {
        let (file, rank) = self.position() - other.position();

        file == 0 || rank == 0 || file.abs() == rank.abs()
    }

    fn position(&self) -> &ChessPosition {
        &self.0
    }
}
