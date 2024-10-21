use minesweeper_rs::*;
fn main() {
    let mut board = Board::build(10, 10, 10).unwrap();
    board.check_tile(0, 0);
    print_board(&board);
}

fn print_board(board: &Board) {
    for y in 0..10 {
        for x in 0..10 {
            let tile = board.get(x, y).unwrap();
            if !tile.is_open() {
                if tile.is_flagged() {
                    print!(" P");
                } else {
                    print!("[]");
                }
            } else if tile.is_mined() {
                print!(" *");
            } else {
                let value = tile.surrounding_mines().unwrap();
                if value == 0 {
                    print!("  ");
                } else {
                    print!(" {}", value);
                }
            }
        }
        println!();
    }
}
