use othello::board::Board;
fn main() {
    println!("Hello, world!");
    let mut board = Board::default();
    board.apply_move(5, 4, false);
    println!("{board}");
}
