mod board;
use board::isola_board::Board;

fn main() {
    let board = Board::new();
    board.display();
}
