mod board;
use board::isola_board::Board;

fn main() {
    let mut board = Board::new();
    board.display();
}
