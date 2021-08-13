use crate::bughouse_board::*;
use crate::error::*;
use chess::{ChessMove, /*Error,*/ Piece, Square};
use std::fmt;
use std::str::FromStr;

/// Represent a ChessMove in memory
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Default, Debug, Hash)]
pub struct BughouseMove {
    source: Option<Square>,
    dest: Square,
    piece: Option<Piece>, // piggybacking on ChessMove's "promotion" for drops also
}

impl BughouseMove {
    /// Create a new chess move, given a source `Square`, a destination `Square`, and an optional
    /// promotion `Piece`
    #[inline]
    pub fn new(
        source: Option<Square>,
        dest: Square,
        piece: Option<Piece>,
    ) -> Self {
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
    ///     BughouseMove::new(Some(Square::E2), Square::E4, None)
    /// );
    /// ```
    pub fn from_ban(
        board: &BughouseBoard,
        move_text: &str,
    ) -> Result<BughouseMove, Error> {
        if let Some(mv) = BughouseMove::from_drop_str(move_text) {
            return if board.is_legal(&mv) {
                Ok(mv)
            } else {
                Err(Error::IllegalMove(move_text.to_string()))
            }
        }

        let mv = ChessMove::from_san(board.get_board(), move_text)?;
        Ok(BughouseMove::new(
                Some(mv.get_source()),
                mv.get_dest(),
                mv.get_promotion(),
                ))
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
        BughouseMove::new(
            Some(mv.get_source()),
            mv.get_dest(),
            mv.get_promotion(),
        )
    }
}

// Upper-case is canonical, but accept both.
fn piece_from_drop_str(drop_str: &str) -> Option<Piece> {
    match drop_str {
        "p" | "P" => Some(Piece::Pawn),
        "n" | "N" => Some(Piece::Knight),
        "b" | "B" => Some(Piece::Bishop),
        "r" | "R" => Some(Piece::Rook),
        "q" | "Q" => Some(Piece::Queen),
        _ => None,
    }
}

impl fmt::Display for BughouseMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (self.source, self.piece) {
            (None, Some(p)) => write!(f, "{}@{}", p, self.dest),
            (Some(s), Some(p)) => write!(f, "{}{}{}", s, self.dest, p),
            (Some(s), None) => write!(f, "{}{}", s, self.dest),
            (None, None) => write!(f, "<Invalid: only dest>: {}", self.dest),
        }
    }
}

/// Convert a BUCI, Bughouse-enabled UCI, move
/// ```
/// use bughouse::{BughouseMove, Square, Piece};
/// use std::str::FromStr;
///
/// let mv = BughouseMove::new(Some(Square::E7), Square::E8, Some(Piece::Queen));
///
/// assert_eq!(BughouseMove::from_str("e7e8q").expect("Valid Move"), mv);
/// ```
impl FromStr for BughouseMove {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(mv) = ChessMove::from_str(s) {
            return Ok(BughouseMove::from_chess_move(&mv));
        } else if let Some(mv) = BughouseMove::from_drop_str(&s) {
            return Ok(mv);
        }
        // Allow something like "e5n" or "bf7"?
        // This author prefers unambiguious BPGN... so
        return Err(Error::MoveParseError(s.to_string()));
    }
}

#[cfg(test)]
pub fn get_mv(mv_str: &str) -> BughouseMove {
    BughouseMove::from_str(mv_str).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use chess::Board;

    #[test]
    pub fn test_make_drop() {
        assert!(BughouseMove::from_str("P@f7").is_ok());
        assert!(BughouseMove::from_str("q@e5").is_ok());
        assert!(BughouseMove::from_str("h@e5").is_err());
    }

    #[test]
    pub fn test_promo() {
        let fen = "rn1qkbnr/pP2pppp/2b5/8/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 1";
        let board = Board::from_str(fen).unwrap();
        let mv = ChessMove::from_str("b7a8q");
        assert!(mv.is_ok());
        assert!(board.legal(mv.unwrap()));
    }
}
