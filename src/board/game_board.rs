use super::{isola_board::Board, turn::Turn};

pub struct GameBoard {
    pub turn: Turn,
    pub board: Board,
}
