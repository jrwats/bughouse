use crate::holdings::*;
use crate::bughouse_move::BughouseMove;
use chess::Board;

/// A representation of one Bughouse board.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct BughouseBoard {
    holdings: Holdings,
    board: Board,
}

impl BughouseBoard {
    /// Get the source square (square the piece is currently on).
    #[inline]
    pub fn get_holdings(&self) -> &Holdings { &self.holdings }

    #[inline]
    pub fn get_board(&self) -> &Board { &self.board }
}

/// Construct the initial position.
impl Default for BughouseBoard {
    #[inline]
    fn default() -> Self {
        BughouseBoard {
            holdings: Holdings::default(),
            board: Board::default(),
        }
    }
}

impl BughouseBoard {
    
    pub fn is_legal(&self, m: BughouseMove) -> bool {
        true
    }
}
