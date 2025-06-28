use othello::board::Board;
fn main() {
    println!("Hello, world!");
    let mut board = Board::default();
    board.apply_move(3, 5, true);
    board.apply_move(2, 5, false);
    board.print(true);
}
