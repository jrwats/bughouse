use crate::bughouse_move::BughouseMove;
use crate::holdings::*;
use chess::{between, BitBoard, Board, Piece, Square, EMPTY};
use std::str::FromStr;

/// A representation of one Bughouse board.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct BughouseBoard {
    holdings: Holdings,
    board: Board,
}

impl BughouseBoard {
    /// Get the source square (square the piece is currently on).
    #[inline]
    pub fn get_holdings(&self) -> &Holdings {
        &self.holdings
    }

    #[inline]
    pub fn get_board(&self) -> &Board {
        &self.board
    }
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
    pub fn in_check(&self) -> bool {
        *self.board.checkers() != EMPTY
    }

    fn blocks_check(&self, drop_sq: Square) -> bool {
        let checkers = self.board.checkers();
        // You can't block double check
        if checkers.popcnt() != 1 {
            return false;
        }
        let checker_sq = checkers.to_square();
        let piece = self.board.piece_on(checker_sq).unwrap();
        if piece != Piece::Knight {
            return false;
        }
        let ksq = self.board.king_square(self.board.side_to_move());
        between(checker_sq, ksq) & BitBoard::from_square(drop_sq) != EMPTY
    }

    pub fn is_legal(&self, mv: BughouseMove) -> bool {
        if mv.get_source() != None {
            if None == mv.get_piece() {
                // Invalid drop
                return false
            }
            // A drop move.
            // 1. Validate player has the piece in "holdings" or "reserves"
            // 2. Ensure it's not atop another piece
            // 3. Ensure the player isn't in check, or
            // 4. The drop blocks check
            let piece = mv.get_piece().unwrap();
            self.holdings.has_piece(self.board.side_to_move(), piece)
                && self.board.piece_on(mv.get_dest()) == None
                && (!self.in_check() || self.blocks_check(mv.get_dest()))
        } else {
            // TODO get off this expensive implementation
            self.board.legal(mv.to_chess_move().unwrap())
        }
    }
}

pub struct BoardParseError;
impl FromStr for BughouseBoard {
    type Err = BoardParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Err(BoardParseError)
        // Ok(BoardBuilder::from_str(value)?.try_into()?)
    }
}
