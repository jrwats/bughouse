use crate::bughouse_board::*;
use chess::{ChessMove, /*Error,*/ Piece, Square};
use std::str::FromStr;

/// Represent a ChessMove in memory
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Default, Debug, Hash)]
pub struct BughouseMove {
    source: Option<Square>,
    dest: Square,
    piece: Option<Piece>, // piggybacking on ChessMove's "promotion" for drops also
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoveParseError;

impl BughouseMove {
    /// Create a new chess move, given a source `Square`, a destination `Square`, and an optional
    /// promotion `Piece`
    #[inline]
    pub fn new(source: Option<Square>, dest: Square, piece: Option<Piece>) -> Self {
        BughouseMove {
            source,
            dest,
            piece,
        }
    }

    /// Get the source square (square the piece is currently on).
    #[inline]
    pub fn get_source(&self) -> Option<Square> {
        self.source
    }

    /// Get the destination square (square the piece is going to).
    #[inline]
    pub fn get_dest(&self) -> Square {
        self.dest
    }

    /// Get the drop or pawn promotion piece (maybe).
    #[inline]
    pub fn get_piece(&self) -> Option<Piece> {
        self.piece
    }

    #[inline]
    pub fn to_chess_move(&self) -> Option<ChessMove> {
        match self.source {
            None => None,
            Some(src) => Some(ChessMove::new(src, self.dest, self.piece)),
        }
    }

    /// Convert a "BAN", Bughouse-extended (Standard) Algebraic Notation move
    /// into a `BughouseMove`.  e.g. drops: "p@f7"
    ///
    /// ```
    /// use bughouse::{BughouseBoard, BughouseMove, Square};
    ///
    /// let board = BughouseBoard::default();
    /// assert_eq!(
    ///     BughouseMove::from_ban(&board, "e4").expect("e4 is valid in the initial position"),
    ///     ChessMove::new(Square::E2, Square::E4, None)
    /// );
    /// ```
    pub fn from_ban(
        board: &BughouseBoard,
        move_text: &str,
    ) -> Result<BughouseMove, MoveParseError> {
        if let Some(mv) = BughouseMove::from_drop_str(move_text) {
            return if board.is_legal(mv) {
                Ok(mv)
            } else {
                Err(MoveParseError)
            };
        }

        match ChessMove::from_san(board.get_board(), move_text) {
            Ok(mv) => Ok(BughouseMove::new(
                Some(mv.get_source()),
                mv.get_dest(),
                mv.get_promotion(),
            )),
            Err(_e) => Err(MoveParseError),
        }
    }

    /// Convert drop algebraic notation to BughouseMove
    /// e.g. drops: "p@f7"
    pub fn from_drop_str(drop_str: &str) -> Option<Self> {
        let mut parts = drop_str.split("@");
        let piece = piece_from_drop_str(parts.next().unwrap_or(""));
        let dest_sq = Square::from_str(parts.next().unwrap_or(""));
        if let (Some(piece), Ok(dest_sq)) = (piece, dest_sq) {
            return Some(BughouseMove::new(
                None, // drops have no source
                dest_sq,
                Some(piece),
            ));
        }
        None
    }

    pub fn from_chess_move(mv: &ChessMove) -> Self {
        BughouseMove::new(Some(mv.get_source()), mv.get_dest(), mv.get_promotion())
    }
}

// Upper-case is canonical, but accept both.
fn piece_from_drop_str(drop_str: &str) -> Option<Piece> {
    match drop_str {
        "n" => Some(Piece::Knight),
        "N" => Some(Piece::Knight),
        "b" => Some(Piece::Bishop),
        "B" => Some(Piece::Bishop),
        "r" => Some(Piece::Rook),
        "R" => Some(Piece::Rook),
        "q" => Some(Piece::Queen),
        "Q" => Some(Piece::Queen),
        _ => None,
    }
}

/// Convert a BUCI, Bughouse-enabled UCI, move
/// ```
/// use bughouse::{BughouseMove, Square, Piece};
/// use std::str::FromStr;
///
/// let mv = BughouseMove::new(Square::E7, Square::E8, Some(Piece::Queen));
///
/// assert_eq!(BughouseMove::from_str("e7e8q").expect("Valid Move"), mv);
/// ```
impl FromStr for BughouseMove {
    type Err = MoveParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(mv) = ChessMove::from_str(s) {
            return Ok(BughouseMove::from_chess_move(&mv));
        } else if s.len() == 4 {
            return BughouseMove::from_drop_str(&s).ok_or(MoveParseError);
        }
        // Allow something like "e5n" or "bf7"?  This author prefers unambiguious BPGN... so
        // not yet
        return Err(MoveParseError);
    }
}
