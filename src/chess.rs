use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    RankParse,
    FileParse,
    Move(MoveError),
}
impl From<Error> for actix_web::Error {
    fn from(err: Error) -> actix_web::Error {
        match err {
            Error::RankParse => actix_web::error::ErrorBadRequest("Invalid rank"),
            Error::FileParse => actix_web::error::ErrorBadRequest("Invalid file"),
            Error::Move(move_error) => match move_error {
                MoveError::EmptyStartingSquare => {
                    actix_web::error::ErrorBadRequest("Starting square is empty")
                }
                MoveError::NotYourTurn => actix_web::error::ErrorBadRequest("Not your turn"),
                MoveError::FriendlyFire => {
                    actix_web::error::ErrorBadRequest("Friendly fire is not allowed")
                }
                MoveError::InvalidPath(r) => {
                    actix_web::error::ErrorBadRequest(format!("Invalid path. Reason: {}", r))
                }
                MoveError::PathIsBlocked => actix_web::error::ErrorBadRequest("Path is blocked"),
                MoveError::NoMotion => actix_web::error::ErrorBadRequest("No motion"),
            },
        }
    }
}

#[cfg(test)]
#[derive(Debug, PartialEq, Eq)]
pub struct SquareOccupied;

#[derive(Debug, PartialEq, Eq)]
pub enum MoveError {
    EmptyStartingSquare,
    NotYourTurn,
    FriendlyFire,
    InvalidPath(&'static str),
    PathIsBlocked,
    NoMotion,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Board {
    pub squares: HashMap<File, HashMap<Rank, Square>>,
    pub state: BoardState,
}

impl Default for Board {
    fn default() -> Self {
        let mut squares = HashMap::new();
        for file in 1..=8 {
            let mut rank_map = HashMap::new();
            for rank in 1..=8 {
                let file = File::try_from(file).unwrap();
                let rank = Rank::try_from(rank).unwrap();
                let position = Position { rank, file };
                let troop = match (rank, file) {
                    (Rank::Two, _) => Some(Troop {
                        piece: Piece::Pawn,
                        color: Color::White,
                        position,
                    }),
                    (Rank::Seven, _) => Some(Troop {
                        piece: Piece::Pawn,
                        color: Color::Black,
                        position,
                    }),
                    (Rank::One, File::A) | (Rank::One, File::H) => Some(Troop {
                        piece: Piece::Rook,
                        color: Color::White,
                        position,
                    }),
                    (Rank::Eight, File::A) | (Rank::Eight, File::H) => Some(Troop {
                        piece: Piece::Rook,
                        color: Color::Black,
                        position,
                    }),
                    (Rank::One, File::B) | (Rank::One, File::G) => Some(Troop {
                        piece: Piece::Knight,
                        color: Color::White,
                        position,
                    }),
                    (Rank::Eight, File::B) | (Rank::Eight, File::G) => Some(Troop {
                        piece: Piece::Knight,
                        color: Color::Black,
                        position,
                    }),
                    (Rank::One, File::C) | (Rank::One, File::F) => Some(Troop {
                        piece: Piece::Bishop,
                        color: Color::White,
                        position,
                    }),
                    (Rank::Eight, File::C) | (Rank::Eight, File::F) => Some(Troop {
                        piece: Piece::Bishop,
                        color: Color::Black,
                        position,
                    }),
                    (Rank::One, File::D) => Some(Troop {
                        piece: Piece::Queen,
                        color: Color::White,
                        position,
                    }),
                    (Rank::Eight, File::D) => Some(Troop {
                        piece: Piece::Queen,
                        color: Color::Black,
                        position,
                    }),
                    (Rank::One, File::E) => Some(Troop {
                        piece: Piece::King,
                        color: Color::White,
                        position,
                    }),
                    (Rank::Eight, File::E) => Some(Troop {
                        piece: Piece::King,
                        color: Color::Black,
                        position,
                    }),
                    _ => None,
                };
                rank_map.insert(rank, Square { troop, position });
            }
            squares.insert(File::try_from(file).unwrap(), rank_map);
        }
        Board {
            squares,
            state: BoardState::ToMove(Color::White),
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board = String::new();
        for rank in 1..=8 {
            for file in 1..=8 {
                let file = File::try_from(file).unwrap();
                let rank = Rank::try_from(rank).unwrap();
                let square = self.squares.get(&file).unwrap().get(&rank).unwrap();
                let troop = match square.troop {
                    Some(ref troop) => match troop.color {
                        Color::White => match troop.piece {
                            Piece::Pawn => '♙',
                            Piece::Knight => '♘',
                            Piece::Bishop => '♗',
                            Piece::Rook => '♖',
                            Piece::Queen => '♕',
                            Piece::King => '♔',
                        },
                        Color::Black => match troop.piece {
                            Piece::Pawn => '♟',
                            Piece::Knight => '♞',
                            Piece::Bishop => '♝',
                            Piece::Rook => '♜',
                            Piece::Queen => '♛',
                            Piece::King => '♚',
                        },
                    },
                    None => '.',
                };
                board.push(troop);
            }
            board.push('\n');
        }
        write!(f, "{}", board)
    }
}

impl Board {
    pub fn move_troop(&mut self, from: Position, to: Position) -> Result<(), Error> {
        let from_square = self
            .squares
            .get(&from.file)
            .unwrap()
            .get(&from.rank)
            .unwrap();
        let to_square = self.squares.get(&to.file).unwrap().get(&to.rank).unwrap();
        if from_square.troop.is_none() {
            return Err(Error::Move(MoveError::EmptyStartingSquare));
        }
        let from_troop = from_square.troop.as_ref().unwrap();
        if !self.state.can_move(from_troop.color) {
            return Err(Error::Move(MoveError::NotYourTurn));
        }
        let mut capturing = false;
        if let Some(troop) = &to_square.troop {
            capturing = true;
            if troop.color == from_troop.color {
                return Err(Error::Move(MoveError::FriendlyFire));
            }
        }

        let path = Self::make_path(from_troop, from, to, capturing)?;
        if from_troop.piece != Piece::Knight {
            for (index, position) in path.iter().enumerate() {
                let square = self
                    .squares
                    .get(&position.file)
                    .unwrap()
                    .get(&position.rank)
                    .unwrap();
                if index != path.len() - 1 && square.troop.is_some() {
                    return Err(Error::Move(MoveError::PathIsBlocked));
                }
            }
        }

        self.squares
            .get_mut(&to.file)
            .unwrap()
            .get_mut(&to.rank)
            .unwrap()
            .troop = Some(from_troop.clone());
        self.squares
            .get_mut(&from.file)
            .unwrap()
            .get_mut(&from.rank)
            .unwrap()
            .troop = None;

        // TODO: Better state management (turn should only toggle if nothing else is triggered by
        // move. i.e., check, checkmate)
        match self.state {
            BoardState::ToMove(Color::White) => self.state = BoardState::ToMove(Color::Black),
            BoardState::ToMove(Color::Black) => self.state = BoardState::ToMove(Color::White),
            _ => todo!(),
        }

        Ok(())
    }

    fn make_path(
        troop: &Troop,
        from: Position,
        to: Position,
        capturing: bool,
    ) -> Result<Vec<Position>, Error> {
        if from == to {
            return Err(Error::Move(MoveError::NoMotion));
        }
        let mut path = vec![];
        let file_diff = (from.file as i8 - to.file as i8).abs();
        let rank_diff = (from.rank as i8 - to.rank as i8).abs();
        match troop.piece {
            Piece::Pawn => {
                if rank_diff == 2 {
                    match troop.color {
                        Color::White => {
                            if from.rank != Rank::Two {
                                return Err(Error::Move(MoveError::InvalidPath(
                                    "Pawn must be on its starting square to move two spaces",
                                )));
                            }
                        }
                        Color::Black => {
                            if from.rank != Rank::Seven {
                                return Err(Error::Move(MoveError::InvalidPath(
                                    "Pawn must be on its starting square to move two spaces",
                                )));
                            }
                        }
                    }
                }
                if rank_diff > 2 {
                    return Err(Error::Move(MoveError::InvalidPath(
                        "Pawn cannot move more than two spaces vertically",
                    )));
                }
                if file_diff > 1 {
                    return Err(Error::Move(MoveError::InvalidPath(
                        "Pawn cannot move more than one space horizontally",
                    )));
                }
                if file_diff == 1 && !capturing {
                    return Err(Error::Move(MoveError::InvalidPath(
                        "Pawn cannot move diagonally without capturing",
                    )));
                }
                if rank_diff > 1 {
                    for rank in (from.rank as u8)..(to.rank as u8) {
                        path.push(Position {
                            file: from.file,
                            rank: Rank::try_from(rank + 2).unwrap(),
                        });
                    }
                }
                path.push(to);
            }
            Piece::Rook => {
                if file_diff > 0 && rank_diff > 0 {
                    return Err(Error::Move(MoveError::InvalidPath(
                        "Rook must move in a purely vertical or horizontal line",
                    )));
                }
                if file_diff > 0 {
                    for file in (from.file as u8)..(to.file as u8) {
                        path.push(Position {
                            // + 2 because File::A as u8 == 0 and we don't want to include the
                            // starting square in the path
                            file: File::try_from(file + 2).unwrap(),
                            rank: from.rank,
                        });
                    }
                }
                if rank_diff > 0 {
                    for rank in (from.rank as u8)..(to.rank as u8) {
                        path.push(Position {
                            file: from.file,
                            rank: Rank::try_from(rank + 2).unwrap(),
                        });
                    }
                }
            }
            Piece::Knight => {
                if file_diff == 0 || rank_diff == 0 {
                    return Err(Error::Move(MoveError::InvalidPath(
                        "Knight must move either two spaces horizontally and one space vertically, or two spaces vertically and one space horizontally",
                    )));
                }
                if file_diff + rank_diff != 3 {
                    return Err(Error::Move(MoveError::InvalidPath(
                        "Knight must move either two spaces horizontally and one space vertically, or two spaces vertically and one space horizontally",
                    )));
                }
                match rank_diff {
                    1 => {
                        for file in (from.file as u8)..=(to.file as u8) {
                            path.push(Position {
                                rank: to.rank,
                                file: File::try_from(file + 1).unwrap(),
                            });
                        }
                    }
                    2 => {
                        for rank in (from.rank as u8)..(to.rank as u8) {
                            path.push(Position {
                                rank: Rank::try_from(rank + 2).unwrap(),
                                file: from.file,
                            });
                        }
                        path.push(to);
                    }
                    _ => unreachable!(),
                }
            }
            Piece::Bishop => {
                if file_diff != rank_diff {
                    return Err(Error::Move(MoveError::InvalidPath(
                        "Bishop must move in a purely diagonal line",
                    )));
                }
                let mut file = from.file as u8;
                for rank in (from.rank as u8)..(to.rank as u8) {
                    path.push(Position {
                        file: File::try_from(file + 2).unwrap(),
                        rank: Rank::try_from(rank + 2).unwrap(),
                    });
                    file += 1;
                }
            }
            Piece::King => {
                if file_diff > 1 || rank_diff > 1 {
                    return Err(Error::Move(MoveError::InvalidPath(
                        "King cannot move more than one space in any direction",
                    )));
                }
                path.push(to);
            }
            Piece::Queen => {
                if file_diff > 0 && rank_diff > 0 && file_diff != rank_diff {
                    return Err(Error::Move(MoveError::InvalidPath(
                        "Queen must move in a purely vertical, horizontal, or diagonal line",
                    )));
                }
                if file_diff == rank_diff {
                    let mut file = from.file as u8;
                    for rank in (from.rank as u8)..(to.rank as u8) {
                        path.push(Position {
                            file: File::try_from(file + 2).unwrap(),
                            rank: Rank::try_from(rank + 2).unwrap(),
                        });
                        file += 1;
                    }
                } else if file_diff > 0 {
                    for file in (from.file as u8)..(to.file as u8) {
                        path.push(Position {
                            file: File::try_from(file + 2).unwrap(),
                            rank: from.rank,
                        });
                    }
                } else if rank_diff > 0 {
                    for rank in (from.rank as u8)..(to.rank as u8) {
                        path.push(Position {
                            file: from.file,
                            rank: Rank::try_from(rank + 2).unwrap(),
                        });
                    }
                }
            }
        }

        Ok(path)
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    #[cfg(test)]
    pub fn remove_troop(&mut self, position: Position) -> Option<Troop> {
        let square = self
            .squares
            .get_mut(&position.file)
            .unwrap()
            .get_mut(&position.rank)
            .unwrap();
        square.troop.take()
    }

    #[cfg(test)]
    pub fn place_troop(&mut self, troop: Troop) -> Result<(), SquareOccupied> {
        if self
            .squares
            .get(&troop.position.file)
            .unwrap()
            .get(&troop.position.rank)
            .unwrap()
            .troop
            .is_some()
        {
            return Err(SquareOccupied);
        }
        self.squares
            .get_mut(&troop.position.file)
            .unwrap()
            .get_mut(&troop.position.rank)
            .unwrap()
            .troop = Some(troop.clone());
        Ok(())
    }

    #[cfg(test)]
    pub fn replace_troop(&mut self, position: Position, troop: Troop) -> Option<Troop> {
        let square = self
            .squares
            .get_mut(&position.file)
            .unwrap()
            .get_mut(&position.rank)
            .unwrap();
        let old_troop = square.troop.clone();
        square.troop = Some(troop);
        old_troop
    }

    #[cfg(test)]
    pub fn set_state(&mut self, state: BoardState) {
        self.state = state;
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}
impl TryFrom<u8> for Rank {
    type Error = Error;
    fn try_from(rank: u8) -> Result<Self, Self::Error> {
        match rank {
            1 => Ok(Rank::One),
            2 => Ok(Rank::Two),
            3 => Ok(Rank::Three),
            4 => Ok(Rank::Four),
            5 => Ok(Rank::Five),
            6 => Ok(Rank::Six),
            7 => Ok(Rank::Seven),
            8 => Ok(Rank::Eight),
            _ => Err(Error::RankParse),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}
impl TryFrom<u8> for File {
    type Error = Error;
    fn try_from(file: u8) -> Result<Self, Self::Error> {
        match file {
            1 => Ok(File::A),
            2 => Ok(File::B),
            3 => Ok(File::C),
            4 => Ok(File::D),
            5 => Ok(File::E),
            6 => Ok(File::F),
            7 => Ok(File::G),
            8 => Ok(File::H),
            _ => Err(Error::FileParse),
        }
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct Position {
    pub file: File,
    pub rank: Rank,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Square {
    pub troop: Option<Troop>,
    pub position: Position,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq)]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Troop {
    pub piece: Piece,
    pub color: Color,
    pub position: Position,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum BoardState {
    ToMove(Color),
    Check(Color),
    Checkmate(Color),
    Stalemate,
    Draw,
}
impl BoardState {
    fn can_move(&self, team: Color) -> bool {
        match self {
            BoardState::ToMove(color) => *color == team,
            BoardState::Check(color) => *color == team,
            _ => false,
        }
    }
}
