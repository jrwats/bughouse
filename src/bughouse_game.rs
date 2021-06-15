use crate::bughouse_board::BughouseBoard;
use std::str::FromStr;

/// A representation of one Bughouse board.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct BughouseGame {
    board_a: BughouseBoard,
    board_b: BughouseBoard,
}

impl BughouseGame {
    pub fn new(board_a: BughouseBoard, board_b: BughouseBoard) -> Self {
        BughouseGame { board_a, board_b }
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

    // Parse 0th rank style FEN holdings (like Lichess' Crazhouse) https://bit.ly/3wx5R3V
    // Future: Add support for suffix holdings (ala chess.com)?
    // Expects only 1 board
    // r2k1r2/pbppNppp/1p2p1nb/1P5N/3N4/4Pn1q/PPP1QP1P/2KR2R1/BrpBBqppN w - - 45 56
    //   The above ^^^ is only one board, (presplit on " | ")
    fn from_str(input_str: &str) -> Result<Self, Self::Err> {
        if let Some((a_str, b_str)) = input_str.split_once(" | ") {
            if let (Ok(board_a), Ok(board_b)) = (
                BughouseBoard::from_str(a_str),
                BughouseBoard::from_str(b_str),
            ) {
                return Ok(BughouseGame::new(board_a, board_b));
            }
        }
        return Err(TableParseError::new("Invalid '|' split".to_string()));
    }
}
