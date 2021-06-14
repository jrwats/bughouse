use crate::bughouse_board::*;
use chess::{ChessMove, /*Error,*/ Piece, Square};

/// Represent a ChessMove in memory
#[derive(Clone, Copy, Eq, PartialOrd, PartialEq, Default, Debug, Hash)]
pub struct BughouseMove {
    source: Option<Square>,
    dest: Square,
    promotion: Option<Piece>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoveParseError;

impl BughouseMove {
    /// Create a new chess move, given a source `Square`, a destination `Square`, and an optional
    /// promotion `Piece`
    #[inline]
    pub fn new(
        source: Option<Square>,
        dest: Square,
        promotion: Option<Piece>
        ) -> Self {
        BughouseMove { source, dest, promotion }
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

    /// Get the promotion piece (maybe).
    #[inline]
    pub fn get_promotion(&self) -> Option<Piece> {
        self.promotion
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
        // TODO handle drops N@e4
        match ChessMove::from_san(board.get_board(), move_text) {
            Ok(mv) => Ok(BughouseMove::new(
                Some(mv.get_source()),
                mv.get_dest(),
                mv.get_promotion(),
            )),
            Err(_e) => Err(MoveParseError),
        }
    }

    pub fn from_chess_move(mv: &ChessMove) -> Self {
        BughouseMove::new(
            Some(mv.get_source()),
            mv.get_dest(),
            mv.get_promotion(),
        )
    }
}

/// Convert a Bughouse-enabled UCI, BUCI (pronounced "Byoosee" like Gary Busey)
/// `String` to a move. If invalid, error
/// ```
/// use bughouse::{BughouseMove, Square, Piece};
/// use std::str::FromStr;
///
/// let mv = BughouseMove::new(Square::E7, Square::E8, Some(Piece::Queen));
///
/// assert_eq!(BughouseMove::from_str("e7e8q").expect("Valid Move"), mv);
/// ```
impl FromStr for ChessMove {
    type Err = MoveParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        match ChessMove::from_str(s) {
            Ok(m) => BughouseMove::from_chess_move(m)
            None(m) => BughouseMove::from_chess_move(m)
        }

        let source = Square::from_str(s.get(0..2).ok_or(Error::InvalidUciMove)?)?;
        let dest = Square::from_str(s.get(2..4).ok_or(Error::InvalidUciMove)?)?;

        let mut promo = None;
        if s.len() == 5 {
            promo = Some(match s.chars().last().ok_or(Error::InvalidUciMove)? {
                'q' => Piece::Queen,
                'r' => Piece::Rook,
                'n' => Piece::Knight,
                'b' => Piece::Bishop,
                _ => return Err(Error::InvalidUciMove),
            });
        }

        Ok(ChessMove::new(source, dest, promo))
    }
}
