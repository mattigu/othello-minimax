use othello::board::Board;
fn main() {
    let mut board = Board::default();
    board.apply_move(1 << 29, true);
    board.apply_move(1 << 21, false);
    board.print(true);
}
