use crate::bughouse_move::BughouseMove;
use crate::holdings::*;
use chess::{between, BitBoard, Board, Color, Piece, Square, EMPTY};
use std::str::FromStr;

/// A representation of one Bughouse board.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct BughouseBoard {
    board: Board,
    holdings: Holdings,
}

impl BughouseBoard {
    pub fn new( board: Board, holdings: Holdings) -> Self {
        BughouseBoard { board, holdings }
    }

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoardParseError;

impl FromStr for BughouseBoard {
    type Err = BoardParseError;

    // Parse 0th rank style FEN holdings (like Lichess' Crazhouse) https://bit.ly/3wx5R3V
    // Future: Add support for suffix holdings (ala chess.com)?
    // Expects only 1 board
    // r2k1r2/pbppNppp/1p2p1nb/1P5N/3N4/4Pn1q/PPP1QP1P/2KR2R1/BrpBBqppN w - - 45 56 
    //   The above ^^^ is only one board, (presplit on " | ")
    fn from_str(input_str: &str) -> Result<Self, Self::Err> {
        // Maybe tolerate only 7 slashes and infer empty holdings?
        if input_str.matches("/").count() != 8 {
            return Err(BoardParseError);
        }
        let (bugboard_str, rest) = input_str.split_at(input_str.find(' ').unwrap());
        let (board_part, holdings_str) = bugboard_str.rsplit_once('/').unwrap();
        let mut board_str = String::from(board_part);
        board_str.push_str(rest);
        let holdings = Holdings::from_str(holdings_str).unwrap();
        let board = Board::from_str(&board_str).unwrap();
        Ok(BughouseBoard::new(board, holdings))
    }
}

#[test]
fn parse_example_board() {
    let bug_board = BughouseBoard::from_str("r2k1r2/pbppNppp/1p2p1nb/1P5N/3N4/4Pn1q/PPP1QP1P/2KR2R1/BrpBBqppN w - - 45 56").unwrap();
    let board = Board::from_str("r2k1r2/pbppNppp/1p2p1nb/1P5N/3N4/4Pn1q/PPP1QP1P/2KR2R1 w - - 45 56");
    let holdings_ex = Holdings::new(&[[0, 1, 3, 0, 0], [3, 0, 0, 1, 1]]);
    assert!(*bug_board.get_holdings() == Holdings::from_str("BrpBBqppN").unwrap());
    assert!(*bug_board.get_holdings() == holdings_ex);

    assert!(bug_board.get_board().side_to_move() == Color::White);
    assert!(*bug_board.get_board() == board.unwrap());
}

#[test]
fn parse_default_board() {
    // Empty holdings
    let bug_board = BughouseBoard::from_str("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR/ w KQkq - 0 1").unwrap();
    let holdings_ex = Holdings::new(&[[0; 5]; 2]);
    let default_board = Board::default();

    assert!(*bug_board.get_holdings() == Holdings::from_str("").unwrap());
    assert!(*bug_board.get_holdings() == holdings_ex);

    assert!(bug_board.get_board().side_to_move() == Color::White);

    println!("default: {:?}", default_board);
    println!("bb def: {:?}", bug_board.get_board());
    assert!(*bug_board.get_board() == default_board);
}
