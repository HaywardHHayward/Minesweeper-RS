use minesweeper_rs::*;
fn main() {
    let mut board = Board::build(10, 10, 10).unwrap();
    board.check_tile(0, 0);
}
