use chess::{Color, Piece, NUM_COLORS}; // , NUM_PIECES};
use std::str::FromStr;

pub const NUM_HELD_PIECE_TYPES: usize = 5; // P, N, B, R, Q

type HeldArray = [[u8; NUM_HELD_PIECE_TYPES]; NUM_COLORS];
fn empty() -> HeldArray {
  [[0; NUM_HELD_PIECE_TYPES]; NUM_COLORS]
}

/// A representation of one Bughouse board.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Holdings {
  holdings: HeldArray,
}

impl Holdings {
  pub fn new(holdings: &HeldArray) -> Self {
    Holdings {
      holdings: *holdings,
    }
  }

  pub fn has_piece(&self, color: Color, piece: Piece) -> bool {
    self.holdings[color.to_index()][piece.to_index()] > 0
  }

  pub fn drop(&mut self, color: Color, piece: Piece) -> Result<(), ()> {
    let color_idx = color.to_index();
    let piece_idx = piece.to_index();
    let cur_val = self.holdings[color_idx][piece_idx];
    if cur_val > 0 {
      self.holdings[color_idx][piece_idx] = cur_val - 1;
      return Ok(());
    }
    return Err(());
  }

  pub fn add(&mut self, color: Color, piece: Piece) {
    let cidx = color.to_index();
    let pidx = piece.to_index();
    self.holdings[cidx][pidx] += 1;
  }
}

/// Construct the initial position.
impl Default for Holdings {
  #[inline]
  fn default() -> Self {
    Holdings { holdings: empty() }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HoldingsParseError(());

impl FromStr for Holdings {
  type Err = HoldingsParseError;

  /// Generate Holdings array from a "BFEN" section (0th rank)
  /// References:
  ///   http://www.czechopen.net/en/festival-tournaments/l-bughouse/rules/
  ///   https://bughousedb.com/Lieven_BPGN_Standard.txt
  ///
  /// Chess.com apparently uses FEN notation with the holdings section following
  fn from_str(value: &str) -> Result<Self, Self::Err> {
    // value
    let mut bfen_holdings = empty();
    let black_idx = Color::Black.to_index();
    let white_idx = Color::White.to_index();
    for p in value.trim().chars() {
      match p {
        'p' => bfen_holdings[black_idx][Piece::Pawn.to_index()] += 1,
        'n' => bfen_holdings[black_idx][Piece::Knight.to_index()] += 1,
        'b' => bfen_holdings[black_idx][Piece::Bishop.to_index()] += 1,
        'r' => bfen_holdings[black_idx][Piece::Rook.to_index()] += 1,
        'q' => bfen_holdings[black_idx][Piece::Queen.to_index()] += 1,
        'P' => bfen_holdings[white_idx][Piece::Pawn.to_index()] += 1,
        'N' => bfen_holdings[white_idx][Piece::Knight.to_index()] += 1,
        'B' => bfen_holdings[white_idx][Piece::Bishop.to_index()] += 1,
        'R' => bfen_holdings[white_idx][Piece::Rook.to_index()] += 1,
        'Q' => bfen_holdings[white_idx][Piece::Queen.to_index()] += 1,
        _ => {
          return Err(HoldingsParseError(()));
        }
      }
    }
    Ok(Holdings {
      holdings: bfen_holdings,
    })
  }
}

#[test]
fn empty_position() {
  let res: Result<Holdings, _> = Holdings::from_str("");
  assert!(res.unwrap() == Holdings::default());
}

// BrpBBqppN
#[test]
fn random_position() {
  let res: Result<Holdings, _> = Holdings::from_str("BrpBBqppN");
  assert!(
    res.unwrap()
      == Holdings {
        holdings: [
          [0, 1, 3, 0, 0], // white
          [3, 0, 0, 1, 1], //black
        ]
      }
  );
}

// BrpBBqppN
#[test]
fn kings_dont_make_sense() {
  let res: Result<Holdings, _> = Holdings::from_str("k");
  assert!(res.is_err());
}
