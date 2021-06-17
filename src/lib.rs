use chess;

pub use chess::*;

mod holdings;
pub use crate::holdings::*;

mod bughouse_move;
pub use crate::bughouse_move::*;

mod bughouse_board;
pub use crate::bughouse_board::*;

mod bughouse_game;
pub use crate::bughouse_game::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
