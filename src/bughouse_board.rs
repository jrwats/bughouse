use crate::bughouse_move::BughouseMove;
use crate::promotions::Promotions;
use crate::holdings::*;
use chess::{
    between, BitBoard, Board, BoardBuilder, BoardStatus, Piece, Square, EMPTY,
};
use std::convert::TryFrom;
use std::str::FromStr;
// use std::fmt;

/// A representation of one Bughouse board.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BughouseBoard {
    board: Board,
    holdings: Holdings,
    promos: Promotions,
}

impl BughouseBoard {
    pub fn new(board: Board, holdings: Holdings, promos: Promotions) -> Self {
        BughouseBoard { board, holdings, promos }
    }

    /// Get the source square (square the piece is currently on).
    #[inline]
    pub fn get_holdings(&self) -> &Holdings {
        &self.holdings
    }

    pub fn holdings(&mut self) -> &mut Holdings {
        &mut self.holdings
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
            promos: Promotions::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidMove {
    bughouse_move: String,
    reason: String,
}
impl InvalidMove {
    pub fn new(mv: &BughouseMove, reason: &str) -> Self {
        InvalidMove {
            bughouse_move: mv.to_string(),
            reason: reason.to_string(),
        }
    }
}

impl BughouseBoard {
    pub fn in_check(&self) -> bool {
        *self.board.checkers() != EMPTY
    }

    fn king_square(&self) -> Square {
        self.board.king_square(self.board.side_to_move())
    }

    pub fn is_mated(&self) -> bool {
        // Disregard checkmates that have an an interposition
        let checkers = self.board.checkers();
        if self.board.status() != BoardStatus::Checkmate {
            return false;
        }
        let sq = checkers.to_square();
        return checkers.popcnt() > 1
            || self.board.piece_on(sq).unwrap() == Piece::Knight
            || between(sq, self.king_square()) == EMPTY;
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
        between(checker_sq, self.king_square()) & BitBoard::from_square(drop_sq)
            != EMPTY
    }

    pub fn make_move(&mut self, mv: &BughouseMove) -> Result<(), InvalidMove> {
        if self.is_legal(mv) {
            if mv.get_source() == None {
                let piece = mv.get_piece().unwrap();
                let color = self.board.side_to_move();
                if let Err(_) = self.holdings.drop(color, piece) {
                    return Err(InvalidMove::new(mv, "Invalid drop"));
                }
                let mut builder = BoardBuilder::from(&self.board);
                builder[mv.get_dest()] = Some((piece, color));
                builder.en_passant(None);
                builder.side_to_move(!self.board.side_to_move());
                if let Ok(board) = Board::try_from(builder) {
                    self.board = board;
                    return Ok(());
                }
                return Err(InvalidMove::new(mv, "Illegal move"));
            } else {
                self.board =
                    self.board.make_move_new(mv.to_chess_move().unwrap());
            }
            return Ok(());
        }
        Err(InvalidMove::new(mv, "Illegal move"))
    }

    pub fn is_legal(&self, mv: &BughouseMove) -> bool {
        if mv.get_source() == None {
            if None == mv.get_piece() {
                // Invalid drop
                return false;
            }
            // A drop move. Ensure that:
            // 1. Player to move has the piece in "holdings" or "reserves"
            // 2. No piece is already there
            // 3. Either (a) the player isn't in check, or
            // 4.        (b) the drop blocks the check
            let piece = mv.get_piece().unwrap();
            self.holdings.has_piece(self.board.side_to_move(), piece)
                && self.board.piece_on(mv.get_dest()) == None
                && (!self.in_check() || self.blocks_check(mv.get_dest()))
        } else {
            // TODO get off this expensive implementation
            self.board.legal(mv.to_chess_move().unwrap())
        }
    }

    pub fn to_bfen(&self) -> String {
        // TODO
       "".to_string()
    }
}

#[test]
fn mated_but_not_in_bughouse() {
    let nonmates = ["3k4/8/8/8/8/8/r/q1K5 w - - - -"];
    for bstr in &nonmates {
        println!("str: {}", bstr);
        assert!(!BughouseBoard::from_str(bstr).unwrap().is_mated());
    }
}

#[test]
fn mated_in_bughouse() {
    let mates = [
        "3k4/8/8/8/8/8/r/qK6 w - - - -",
        "3k2r1/8/8/8/8/8/5nr1/7K w - - - -",
    ];
    for bstr in &mates {
        println!("str: {}", bstr);
        assert!(BughouseBoard::from_str(bstr).unwrap().is_mated());
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
    // Dropping support for clock times (in seconds) here as its better handled at the server level
    fn from_str(input_str: &str) -> Result<Self, BoardParseError> {
        // Tolerate only 7 slashes and infer empty holdings
        let count = input_str.matches("/").count();
        if count < 7 || count > 8 {
            return Err(BoardParseError);
        }
        let (bugboard_str, rest) =
            input_str.split_at(input_str.find(' ').unwrap());
        let (board_part, holdings_str) = if count == 8 {
            bugboard_str.rsplit_once('/').unwrap()
        } else {
            (bugboard_str, "")
        };
        let mut board_str = String::from(board_part.replace('~', ""));
        board_str.push_str(rest);
        let holdings = Holdings::from_str(holdings_str).unwrap();
        let board = Board::from_str(&board_str).unwrap();
        let promotions = Promotions::from_fen(board_part);
        Ok(BughouseBoard::new(board, holdings, promotions))
    }
}

// ICS
//      
//    +-------------------------------+
// 8  | *R|   |   | *R|   |   | *K|   |
//    |---+---+---+---+---+---+---+---|
// 7  | *P| Q |   | *N| *B| *P| *P| *P|     White Moves : 'Bd5     (0:31)'
//    |---+---+---+---+---+---+---+---|
// 6  |   |   |   |   | *Q|   |   |   |
//    |---+---+---+---+---+---+---+---|
// 5  |   |   |   | B | P |   |   |   |     
//    |---+---+---+---+---+---+---+---|
// 4  |   |   | P |   |   |   |   |   |     
//    |---+---+---+---+---+---+---+---|
// 3  |   | P |   |   |   |   |   |   |     
//    |---+---+---+---+---+---+---+---|
// 2  | P |   |   |   |   | P | P | P |     
//    |---+---+---+---+---+---+---+---|
// 1  | R |   | B |   | R |   | K |   |
//    +-------------------------------+
//      a   b   c   d   e   f   g   h
// impl fmt::Display for BughouseBoard {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         
//         
//     }
// }

#[test]
fn parse_promoted_piece() {
    let bug_board =
        BughouseBoard::from_str("Q~4rk1/8/8/8/8/8/8/R3K2R w KQ - 45 60")
            .unwrap();
    let board = Board::from_str("Q4rk1/8/8/8/8/8/8/R3K2R w KQ - 0 1").unwrap();
    let holdings_ex = Holdings::new(&[[0; 5]; 2]);
    assert!(*bug_board.get_holdings() == Holdings::from_str("").unwrap());
    assert!(*bug_board.get_holdings() == holdings_ex);
    assert!(bug_board.get_board().side_to_move() == chess::Color::White);
    assert!(*bug_board.get_board() == board);
}

#[test]
fn parse_example_board() {
    let bug_board = BughouseBoard::from_str("r2k1r2/pbppNppp/1p2p1nb/1P5N/3N4/4Pn1q/PPP1QP1P/2KR2R1/BrpBBqppN w - - 45 56").unwrap();
    let board = Board::from_str(
        "r2k1r2/pbppNppp/1p2p1nb/1P5N/3N4/4Pn1q/PPP1QP1P/2KR2R1 w - - 0 1",
    );
    let holdings_ex1 = Holdings::from_str("BrpBBqppN").unwrap();
    assert!(*bug_board.get_holdings() == holdings_ex1);
    let holdings_ex = Holdings::new(&[[0, 1, 3, 0, 0], [3, 0, 0, 1, 1]]);
    assert!(*bug_board.get_holdings() == holdings_ex);
    assert!(*bug_board.get_board() == board.unwrap());
}

#[test]
fn parse_default_board() {
    // Empty holdings
    let bug_board = BughouseBoard::from_str(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR/ w KQkq - 0 1",
    )
    .unwrap();
    let default_board = Board::default();
    assert!(*bug_board.get_holdings() == Holdings::from_str("").unwrap());
    assert!(*bug_board.get_board() == default_board);
}

#[cfg(test)]
use crate::bughouse_move::get_mv;

#[cfg(test)]
use chess::Color;

#[test]
fn make_some_moves() {
    let mut board = BughouseBoard::default();
    board.make_move(&get_mv("e2e4")).unwrap();
    assert!(board.board.side_to_move() == Color::Black);
    assert!(board.make_move(&get_mv("e7e5")).is_ok());
    assert!(board.board.side_to_move() == Color::White);
}

#[test]
fn catch_invalid_moves() {
    let mut board = BughouseBoard::default();
    board.make_move(&get_mv("e2e4")).unwrap();
    assert!(board.make_move(&get_mv("e2e4")).is_err());
}

#[test]
fn test_holdings_mutability() {
    let mut board = BughouseBoard::default();
    let expected_holdings = Holdings::new(&[[0, 1, 0, 0, 0], [0; 5]]);
    {
        let holdings = board.holdings();
        holdings.add(Color::White, Piece::Knight);
        assert!(*holdings == expected_holdings);
    }
    assert!(*board.get_holdings() == expected_holdings);
}
