mod solution;
use std::convert::TryInto;

use solution::ChessGame;
use solution::Piece::*;
use solution::PieceType::*;
use solution::Turn;
use solution::Error;
use solution::Position;

fn main() {
    let mut game = ChessGame::new_game();
    let position: Position = "e1".try_into().unwrap();
    println!("{:?}", game.board);
    println!("{:?}", position);

    let text = match &game.get_field(position) {
        Some(White(Rook)) => "white rook",
        Some(White(Knight)) => "white knight",
        Some(White(Bishop)) => "white bishop",
        Some(White(Queen)) => "white queen",
        Some(White(King)) => "white king",
        Some(White(Pawn)) => "white pawn",
        Some(Black(Rook)) => "black rook",
        Some(Black(Knight)) => "black knight",
        Some(Black(Bishop)) => "black bishop",
        Some(Black(Queen)) => "black queen",
        Some(Black(King)) => "black king",
        Some(Black(Pawn)) => "black pawn",
        None => "empty field",
    };

    let text3 = match game.make_move("a1".try_into().unwrap(), "a2".try_into().unwrap()) {
        Ok(None) => "valid move",
        Ok(Some(White(Pawn))) => "valid move & white pawn taken",
        Err(Error::InvalidMove) => "invalid move",
        _ => "other",
    };
    println!("{}", text3);
    println!("{:?}", game.board);
}