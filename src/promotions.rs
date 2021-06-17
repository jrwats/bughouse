use chess::{
    BitBoard, ChessMove, Color, File, Rank, Square, EMPTY, NUM_COLORS,
};

type PromoArray = [BitBoard; NUM_COLORS];
fn empty() -> PromoArray {
    [EMPTY; NUM_COLORS]
}

/// A representation of tracking squares that have a piece promoted from a pawn.
///
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Promotions {
    promos: PromoArray,
}

impl Promotions {
    pub fn new(promos: &PromoArray) -> Self {
        Promotions { promos: *promos }
    }

    pub fn is_promo(&self, color: Color, sq: Square) -> bool {
        self.promos[color.to_index()] & BitBoard::from_square(sq) != EMPTY
    }

    pub fn add_square(&mut self, color: Color, sq: Square) {
        self.promos[color.to_index()] |= BitBoard::from_square(sq);
    }

    pub fn clear_square(&mut self, color: Color, sq: Square) {
        self.promos[color.to_index()] &= !BitBoard::from_square(sq);
    }

    pub fn record_move(&mut self, mover: Color, mv: ChessMove) {
        if mv.get_promotion().is_some() {
            self.add_square(mover, mv.get_dest());
            return;
        }
        if self.is_promo(mover, mv.get_source()) {
            self.clear_square(mover, mv.get_source());
            self.add_square(mover, mv.get_dest());
        }
        // If a promoted piece was captured, clear it now.  (BughouseBoard is expected to already
        // update holdings correctly before callings this)
        self.clear_square(!mover, mv.get_dest());
    }

    pub fn from_fen(fen: &str) -> Promotions {
        let mut bit_boards = [EMPTY; 2];
        for (row, rank) in fen.split('/').zip((0..8_usize).rev()) {
            let mut file_idx = 0;
            let mut last_color = Color::White;
            for ch in row.chars() {
                match ch {
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => {
                        file_idx += (ch as usize) - ('0' as usize)
                    }
                    'p' | 'n' | 'b' | 'r' | 'q' | 'k' => {
                        last_color = Color::Black;
                        file_idx += 1;
                    }
                    'P' | 'N' | 'B' | 'R' | 'Q' | 'K' => {
                        last_color = Color::White;
                        file_idx += 1;
                    }
                    '~' => {
                        let file = File::from_index(file_idx - 1);
                        let sq =
                            Square::make_square(Rank::from_index(rank), file);
                        bit_boards[last_color.to_index()] |=
                            BitBoard::from_square(sq);
                    }
                    _ => {}
                }
            }
        }
        Promotions::new(&bit_boards)
    }
}

/// Construct the initial position.
impl Default for Promotions {
    #[inline]
    fn default() -> Self {
        Promotions { promos: empty() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromotionsParseError {
    fen: String,
}

#[test]
fn parse_promos() {
    let promos = Promotions::from_fen("Q~4rk1/8/8/8/8/8/8/R3K2R");
    println!("promos: {:?}", promos);
    println!("sq: {:?}", promos.promos[0].to_square());
    for (i, r) in (0..8_usize).zip((0..8_usize).map(|i| Rank::from_index(i))) {
        println!("idx: {}, rank: {:?}", i, r);
    }
    assert!(promos.is_promo(Color::White, Square::A8));
}
