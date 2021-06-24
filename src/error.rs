use thiserror::Error;

fn color_to_str(c: chess::Color) -> String {
  match c {
      chess::Color::White => "White".to_string(),
      chess::Color::Black => "Black".to_string(),
  }
}

#[derive(Clone, Debug, Error)]
pub enum Error {
    /// The FEN string is invalid
    #[error("Invalid Game BFEN")]
    GameParseError(String),

    #[error("Invalid Board BFEN: {0}")]
    BoardParseError(String),

    #[error("Illegal move: {0}")]
    IllegalMove(String),

    #[error("Can't parse move: {0}")]
    MoveParseError(String),
    
    #[error("Unheld Drop: {} {1}", color_to_str(*.0))]
    UnheldDrop(chess::Color, chess::Piece),

    #[error("Invalid holdings: {0}")]
    HoldingsParseError(String),

    #[error("Chess Error: {0}")]
    Chess(chess::Error),
}

impl From<chess::Error> for Error {
    fn from(err: chess::Error) -> Self {
        Error::Chess(err)
    }
}
