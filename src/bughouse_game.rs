use crate::bughouse_board::BughouseBoard;
use crate::bughouse_move::BughouseMove;
use crate::error::*;
use chess::Piece;
use std::str::FromStr;
// use std::fmt;

#[derive(PartialEq, Eq, Ord, PartialOrd, Copy, Clone, Debug, Hash)]
pub enum BoardID {
    A,
    B,
}

// For moving from index to BoardID
pub const BOARD_IDS: [BoardID; 2] = [BoardID::A, BoardID::B];

impl BoardID {
    /// Convert the `BoardName ` to a `usize` for table lookups.
    #[inline]
    pub fn to_index(&self) -> usize {
        *self as usize
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

    // TODO
    // pub fn is_sane(&self) -> bool {
    // }

    pub fn make_move(
        &mut self,
        name: BoardID,
        mv: &BughouseMove,
    ) -> Result<(), Error> {
        let bug_board = &mut self.boards[name.to_index()];
        let chess_board = bug_board.get_board();
        let dest = mv.get_dest();
        let captured_piece = chess_board.piece_on(dest);
        let opp = !chess_board.side_to_move();
        let is_promo = bug_board.get_promos().is_promo(opp, dest);
        bug_board.make_move(mv)?;
        if let Some(piece) = captured_piece {
            let other_board = &mut self.boards[1 - name.to_index()];
            other_board
                .holdings()
                .add(opp, if is_promo { Piece::Pawn } else { piece });
        }
        return Ok(());
    }
}

impl FromStr for BughouseGame {
    type Err = Error;

    /// Note: ignores time input, so flagging will not be handled.
    fn from_str(input_str: &str) -> Result<Self, Self::Err> {
        if let Some((a_str, b_str)) = input_str.split_once(" | ") {
            let board_a = BughouseBoard::from_str(a_str)?;
            let board_b = BughouseBoard::from_str(b_str)?;
            Ok(BughouseGame::new(board_a, board_b))
        } else {
            Err(Error::GameParseError(format!("Invalid ' | ' split: {}", input_str)))
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
mod test {
    use super::*;
    use crate::bughouse_move::get_mv;
    use crate::Holdings;
    use crate::Promotions;
    use chess::{BitBoard, Square, EMPTY};

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

    #[test]
    fn tracking_promos() {
        let bfen = format!(
            "{} | {}",
            "4k3/7P/8/q78/8/PPPPPPP/RNBQKBNR/ w - - - -",
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR/nnbbrrpppppppp w - - - -",
            );
        let mut game = BughouseGame::from_str(&bfen).unwrap();
        assert!(game.make_move(BoardID::A, &get_mv("h7h8q")).is_ok());
        let expected_promos =
            Promotions::new(&[BitBoard::from_square(Square::H8), EMPTY]);
        assert!(*game.get_board(BoardID::A).get_promos() == expected_promos);
        assert!(game.make_move(BoardID::A, &get_mv("e8e7")).is_ok());
        assert!(game.make_move(BoardID::A, &get_mv("h8h5")).is_ok());
        let expected_promos =
            Promotions::new(&[BitBoard::from_square(Square::H5), EMPTY]);
        assert!(*game.get_board(BoardID::A).get_promos() == expected_promos);
        println!("holdings: {:?}", game.get_board(BoardID::B).get_holdings());
        assert!(
            *game.get_board(BoardID::B).get_holdings()
                == Holdings::new(&[[0; 5], [8, 2, 2, 2, 0]])
        );
        assert!(game.make_move(BoardID::A, &get_mv("a5h5")).is_ok());

        // Queen goes back as pawn
        assert!(
            *game.get_board(BoardID::B).get_holdings()
                == Holdings::new(&[[1, 0, 0, 0, 0], [8, 2, 2, 2, 0]])
        );
        let expected_promos = Promotions::new(&[EMPTY, EMPTY]);
        assert!(*game.get_board(BoardID::A).get_promos() == expected_promos);
    }
}
