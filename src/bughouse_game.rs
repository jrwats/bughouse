use crate::bughouse_board::{BughouseBoard, InvalidMove};
use crate::bughouse_move::BughouseMove;
use chess::Piece;
use std::str::FromStr;

#[derive(PartialEq, Eq, Ord, PartialOrd, Copy, Clone, Debug, Hash)]
pub enum BoardID {
    A,
    B,
}

impl BoardID {
    /// Convert the `BoardName ` to a `usize` for table lookups.
    #[inline]
    pub fn to_index(&self) -> usize {
        *self as usize
    }
}

impl BoardID {
    pub fn other(name: BoardID) -> Self {
        match name {
            BoardID::A => BoardID::B,
            BoardID::B => BoardID::A,
        }
    }
}

/// A representation of one Bughouse board.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct BughouseGame {
    boards: [BughouseBoard; 2],
}

impl Default for BughouseGame {
    #[inline]
    fn default() -> Self {
        BughouseGame::new(BughouseBoard::default(), BughouseBoard::default())
    }
}

impl BughouseGame {
    pub fn new(a: BughouseBoard, b: BughouseBoard) -> Self {
        BughouseGame { boards: [a, b] }
    }

    pub fn get_board(&self, id: BoardID) -> &BughouseBoard {
        &self.boards[id.to_index()]
    }


    pub fn make_move(
        &mut self,
        name: BoardID,
        mv: &BughouseMove,
    ) -> Result<(), InvalidMove> {
        let bug_board = &mut self.boards[name.to_index()];
        let chess_board = bug_board.get_board();
        let dest = mv.get_dest();
        let captured_piece = chess_board.piece_on(dest);
        let opp = !chess_board.side_to_move();
        let is_promo = bug_board.get_promos().is_promo(opp, dest);
        bug_board.make_move(mv)?;
        if let Some(piece) = captured_piece {
            let other_board = &mut self.boards[1 - name.to_index()];
            other_board.holdings().add(opp, if is_promo { Piece::Pawn } else { piece });
        }
        return Ok(());
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableParseError {
    msg: String,
}

impl TableParseError {
    pub fn new(msg: String) -> Self {
        TableParseError { msg }
    }
}

impl FromStr for BughouseGame {
    type Err = TableParseError;

    /// Note: ignores time input, so flagging will not be handled.
    fn from_str(input_str: &str) -> Result<Self, Self::Err> {
        if let Some((a_str, b_str)) = input_str.split_once(" | ") {
            match (
                BughouseBoard::from_str(a_str),
                BughouseBoard::from_str(b_str),
            ) {
                (Ok(board_a), Ok(board_b)) => {
                    Ok(BughouseGame::new(board_a, board_b))
                }
                _ => Err(TableParseError::new("Bad inner BFEN".to_string())),
            }
        } else {
            Err(TableParseError::new("Invalid '|' split".to_string()))
        }
    }
}

// Pretty print each board
// impl fmt::Display for BughouseMove {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//
//     }
// }

#[cfg(test)]
use crate::Holdings;

#[cfg(test)]
use crate::bughouse_move::get_mv;

#[test]
fn opening_game() {
    let mut game = BughouseGame::default();
    println!("beg a: {:?}", game.boards[0]);
    game.make_move(BoardID::A, &get_mv("e2e4")).unwrap();
    println!("end a: {:?}", game.boards[0]);
    let color = game.boards[0].get_board().side_to_move();
    assert!(color == chess::Color::Black);
    game.make_move(BoardID::A, &get_mv("e7e5")).unwrap();
}

#[test]
fn short_bug_game() {
    let mut game = BughouseGame::default();
    let moves = [
        (BoardID::A, get_mv("e2e4")), // e4
        (BoardID::A, get_mv("e7e5")), // e5
        (BoardID::A, get_mv("f1c4")), // Bc4
        (BoardID::A, get_mv("b8c6")), // Nc6
        (BoardID::A, get_mv("c4f7")), // Bxf7
        (BoardID::A, get_mv("e8f7")), // Kxf7
        (BoardID::A, get_mv("g1f3")), // Nf3
        (BoardID::A, get_mv("f7e8")), // Ne7
        (BoardID::A, get_mv("f3g5")), // Ng5+
        (BoardID::A, get_mv("g8e7")), // Ke8
        (BoardID::B, get_mv("e2e4")), // e4
        (BoardID::B, get_mv("d7d5")), // d5
        (BoardID::B, get_mv("e4d5")), // exd5
        (BoardID::B, get_mv("d8d5")), // Qxd5
    ];
    for (name, mv) in &moves {
        game.make_move(*name, &mv).unwrap();
    }
    assert!(!game.boards[0].is_mated());
    // Each white player has a pawn
    let expected_holdings = Holdings::new(&[[1, 0, 0, 0, 0]; 2]);
    assert!(*game.boards[0].get_holdings() == expected_holdings);
    assert!(game.make_move(BoardID::A, &get_mv("P@f7")).is_ok());
    assert!(game.boards[0].is_mated());
}
