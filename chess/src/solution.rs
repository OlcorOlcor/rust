use core::convert::TryFrom;
use core::convert::TryInto;

#[derive(Debug, Clone, Copy)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Debug, Clone, Copy)]
pub enum Piece {
    White(PieceType),
    Black(PieceType),
}

enum Color {
    White,
    Black
}

#[derive(Debug)]
pub struct Board {
    pub fields: [[Option<Piece>; 8]; 8],
}

impl Board {
    pub fn new_empty() -> Board {
        Board {
            fields: [[None; 8]; 8],
        }
    }
    pub fn replace_field(&mut self, row: usize, col: usize, value: Option<Piece>) -> Result<Option<Piece>, Error> {
        if !(0..8).contains(&row) {
            Err(Error::InvalidRow)
        } else if !(0..8).contains(&col) {
            Err(Error::InvalidColumn)
        } else {
            let orig = self.fields[row][col];
            self.fields[row][col] = value;
            Ok(orig)
        }
    }
}
pub struct ChessGame {
    pub board: Board,
    current: Turn
}

impl ChessGame {
    fn home_row(&mut self, color: Color) {
        match color {
            Color::White => {
                let _ = self.board.replace_field(0, 0, Some(Piece::White(PieceType::Rook)));
                let _ = self.board.replace_field(0, 7, Some(Piece::White(PieceType::Rook)));
                let _ = self.board.replace_field(0, 1, Some(Piece::White(PieceType::Knight)));
                let _ = self.board.replace_field(0, 6, Some(Piece::White(PieceType::Knight)));
                let _ = self.board.replace_field(0, 2, Some(Piece::White(PieceType::Bishop)));
                let _ = self.board.replace_field(0, 5, Some(Piece::White(PieceType::Bishop)));
                let _ = self.board.replace_field(0, 3, Some(Piece::White(PieceType::Queen)));
                let _ = self.board.replace_field(0, 4, Some(Piece::White(PieceType::King)));
            }
            Color::Black => {
                let _ = self.board.replace_field(7, 0, Some(Piece::Black(PieceType::Rook)));
                let _ = self.board.replace_field(7, 7, Some(Piece::Black(PieceType::Rook)));
                let _ = self.board.replace_field(7, 1, Some(Piece::Black(PieceType::Knight)));
                let _ = self.board.replace_field(7, 6, Some(Piece::Black(PieceType::Knight)));
                let _ = self.board.replace_field(7, 2, Some(Piece::Black(PieceType::Bishop)));
                let _ = self.board.replace_field(7, 5, Some(Piece::Black(PieceType::Bishop)));
                let _ = self.board.replace_field(7, 3, Some(Piece::Black(PieceType::Queen)));
                let _ = self.board.replace_field(7, 4, Some(Piece::Black(PieceType::King)));
            }
        };
        
    }
    
    fn pawn_row(&mut self, color: Color) {
        match color {
            Color::White => {
                for col in 0..8 {
                    let _ = self.board.replace_field(1, col, Some(Piece::White(PieceType::Pawn)));
                }
            },
            Color::Black => {
                for col in 0..8 {
                    let _ = self.board.replace_field(6, col, Some(Piece::Black(PieceType::Pawn)));
                }
            }
        };
    }

    pub fn new_game() -> ChessGame {
        let mut game = ChessGame {board: Board::new_empty(), current: Turn::WhitePlays};
        game.pawn_row(Color::White);
        game.pawn_row(Color::Black);
        game.home_row(Color::White);
        game.home_row(Color::Black);
        game
    }

    pub fn get_field(&self, position: Position) -> Option<Piece> {
        self.board.fields[position.row][position.col]
    }

    pub fn current_player(&self) -> Turn {
        self.current
    }

    pub fn make_move(&mut self, src: Position, dst: Position) -> Result<Option<Piece>, Error> {
        return match self.board.replace_field(src.row, src.col, None) {
            Ok(orig) => {
                match self.board.replace_field(dst.row, dst.col, orig) {
                    Ok(orig) => {
                        match self.current {
                            Turn::BlackPlays => { 
                                match orig {
                                    None => { self.current = Turn::WhitePlays; return Ok(orig); },
                                    _ => match orig.unwrap() {
                                        Piece::Black(_) => { return Err(Error::InvalidMove); },
                                        _ => { self.current = Turn::WhitePlays; return Ok(orig); }
                                    }
                                }
                            }
                            Turn::WhitePlays => { 
                                match orig {
                                    None => { self.current = Turn::BlackPlays; return Ok(orig); },
                                    _ => match orig.unwrap() { 
                                        Piece::White(_) => { return Err(Error::InvalidMove); },
                                        _ => { self.current = Turn::BlackPlays; return Ok(orig); }
                                    }
                                }
                             }
                        }
                    }
                    Err(_) => Err(Error::InvalidMove)
                }
            }
            Err(_) => Err(Error::InvalidMove)
        }
    }
}

#[derive(Debug)]
pub struct Position {
    pub row: usize,
    pub col: usize
}

impl TryFrom<&str> for Position {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let [letter, number]: [char; 2] = value.chars()
        .collect::<Vec<_>>()
        .try_into()
        .map_err(|_| Error::InvalidPosition)?;
        let row = "12345678".chars().position(|n| n == number)
            .ok_or(Error::InvalidPosition)?;
        let col = "abcdefgh".chars().position(|c| c == letter)
            .ok_or(Error::InvalidPosition)?;
        let position = Position {row, col};
        Ok(position)
    }
}
#[derive(Clone, Copy)]
pub enum Turn {
    WhitePlays,
    BlackPlays
}

#[derive(Debug)]
pub enum Error {
    InvalidRow,
    InvalidColumn,
    InvalidPosition,
    InvalidMove
}


