mod tests;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    RankParse,
    FileParse,
    Move(MoveError),
}
#[cfg(feature = "actix")]
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
                MoveError::Other => actix_web::error::ErrorBadRequest("Invalid Path"),
                MoveError::NoMotion => actix_web::error::ErrorBadRequest("No motion"),
            },
        }
    }
}

#[cfg(feature = "lambda")]
impl From<Error> for lambda_runtime::Error {
    fn from(err: Error) -> lambda_runtime::Error {
        match err {
            Error::RankParse => lambda_runtime::Error::from("Invalid rank"),
            Error::FileParse => lambda_runtime::Error::from("Invalid file"),
            Error::Move(move_error) => match move_error {
                MoveError::EmptyStartingSquare => {
                    lambda_runtime::Error::from("Starting square is empty")
                }
                MoveError::NotYourTurn => lambda_runtime::Error::from("Not your turn"),
                MoveError::FriendlyFire => {
                    lambda_runtime::Error::from("Friendly fire is not allowed")
                }
                MoveError::Other => lambda_runtime::Error::from("Invalid Path"),
                MoveError::NoMotion => lambda_runtime::Error::from("No motion"),
            },
        }
    }
}

#[cfg(any(test, debug_assertions))]
#[derive(Debug, PartialEq, Eq)]
pub struct SquareOccupied;

#[derive(Debug, PartialEq, Eq)]
pub enum MoveError {
    EmptyStartingSquare,
    NotYourTurn,
    FriendlyFire,
    Other,
    NoMotion,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

#[cfg(feature = "actions")]
impl Board {
    pub fn get_square(&self, position: &Position) -> &Square {
        self.squares
            .get(&position.file)
            .unwrap()
            .get(&position.rank)
            .unwrap()
    }

    pub fn get_mut_square(&mut self, position: &Position) -> &mut Square {
        self.squares
            .get_mut(&position.file)
            .unwrap()
            .get_mut(&position.rank)
            .unwrap()
    }

    pub fn move_troop(&mut self, from: Position, to: Position) -> Result<(), Error> {
        let from_square = self.get_square(&from);
        let to_square = self.get_square(&to);
        if from_square.troop.is_none() {
            return Err(Error::Move(MoveError::EmptyStartingSquare));
        }
        let from_troop = from_square.troop.as_ref().unwrap();
        if !self.state.can_move(from_troop.color) {
            return Err(Error::Move(MoveError::NotYourTurn));
        }
        if let Some(troop) = &to_square.troop {
            if troop.color == from_troop.color {
                return Err(Error::Move(MoveError::FriendlyFire));
            }
        }

        match from_troop.piece {
            Piece::Pawn | Piece::Rook | Piece::Knight => {
                dbg!(self.valid_moves(from_troop));
                if !self.valid_moves(from_troop).contains(&to_square) {
                    return Err(Error::Move(MoveError::Other));
                }
            }
            _ => {
                let path = Self::make_path(from_troop, from, to)?;
                for (index, position) in path.iter().enumerate() {
                    let square = self.get_square(position);
                    if index != path.len() - 1 && square.troop.is_some() {
                        return Err(Error::Move(MoveError::Other));
                    }
                }
            }
        }

        self.get_mut_square(&to).troop = Some(from_troop.clone());
        self.get_mut_square(&from).troop = None;

        self.get_mut_square(&to).troop.as_mut().unwrap().position = to;

        // TODO: Better state management (turn should only toggle if nothing else is triggered by
        // move. i.e., check, checkmate)
        match self.state {
            BoardState::ToMove(Color::White) => self.state = BoardState::ToMove(Color::Black),
            BoardState::ToMove(Color::Black) => self.state = BoardState::ToMove(Color::White),
            _ => todo!(),
        }

        Ok(())
    }

    fn make_path(troop: &Troop, from: Position, to: Position) -> Result<Vec<Position>, Error> {
        if from == to {
            return Err(Error::Move(MoveError::NoMotion));
        }
        let mut path = vec![];
        let file_diff = (from.file as i8 - to.file as i8).abs();
        let rank_diff = (from.rank as i8 - to.rank as i8).abs();
        let file_iter = match File::cmp(&from.file, &to.file) {
            std::cmp::Ordering::Less => {
                either::Either::Left(from.file as u8 + 2..to.file as u8 + 2)
            }
            std::cmp::Ordering::Equal => {
                either::Either::Left(from.file as u8 + 2..to.file as u8 + 2)
            }
            std::cmp::Ordering::Greater => {
                either::Either::Right((to.file as u8..from.file as u8 + 1).rev())
            }
        };
        let rank_iter = match Rank::cmp(&from.rank, &to.rank) {
            std::cmp::Ordering::Less => {
                either::Either::Left(from.rank as u8 + 2..to.rank as u8 + 2)
            }
            std::cmp::Ordering::Equal => {
                either::Either::Left(from.rank as u8 + 2..to.rank as u8 + 2)
            }
            std::cmp::Ordering::Greater => {
                either::Either::Right((to.rank as u8..from.rank as u8 + 1).rev())
            }
        };
        match troop.piece {
            Piece::Bishop => {
                if file_diff != rank_diff {
                    return Err(Error::Move(MoveError::Other));
                }
                let mut file = match File::cmp(&from.file, &to.file) {
                    std::cmp::Ordering::Less => from.file as u8 + 2,
                    std::cmp::Ordering::Equal => from.file as u8 + 2,
                    std::cmp::Ordering::Greater => from.file as u8,
                };
                for rank in rank_iter {
                    path.push(Position {
                        file: File::try_from(file).unwrap(),
                        rank: Rank::try_from(rank).unwrap(),
                    });
                    match File::cmp(&from.file, &to.file) {
                        std::cmp::Ordering::Less => file += 1,
                        std::cmp::Ordering::Equal => file += 1,
                        std::cmp::Ordering::Greater => file -= 1,
                    }
                }
            }
            Piece::King => {
                if file_diff > 1 || rank_diff > 1 {
                    return Err(Error::Move(MoveError::Other));
                }
                path.push(to);
            }
            Piece::Queen => {
                if file_diff > 0 && rank_diff > 0 && file_diff != rank_diff {
                    return Err(Error::Move(MoveError::Other));
                }
                if file_diff == rank_diff {
                    let mut file = match File::cmp(&from.file, &to.file) {
                        std::cmp::Ordering::Less => from.file as u8 + 2,
                        std::cmp::Ordering::Equal => from.file as u8 + 2,
                        std::cmp::Ordering::Greater => from.file as u8,
                    };
                    for rank in rank_iter {
                        path.push(Position {
                            file: File::try_from(file).unwrap(),
                            rank: Rank::try_from(rank).unwrap(),
                        });
                        match File::cmp(&from.file, &to.file) {
                            std::cmp::Ordering::Less => file += 1,
                            std::cmp::Ordering::Equal => file += 1,
                            std::cmp::Ordering::Greater => file -= 1,
                        }
                    }
                } else if file_diff > 0 {
                    for file in file_iter {
                        path.push(Position {
                            file: File::try_from(file).unwrap(),
                            rank: from.rank,
                        });
                    }
                } else if rank_diff > 0 {
                    for rank in rank_iter {
                        path.push(Position {
                            file: from.file,
                            rank: Rank::try_from(rank).unwrap(),
                        });
                    }
                }
            }
            _ => unreachable!(),
        }

        Ok(path)
    }

    pub fn valid_moves(&self, troop: &Troop) -> Vec<&Square> {
        let mut valid_moves = vec![];

        match troop.piece {
            Piece::Pawn => match troop.color {
                Color::White => {
                    let position_in_front = match troop.position.rank {
                        Rank::Eight => None,
                        _ => Some(Position {
                            file: troop.position.file,
                            rank: Rank::try_from(troop.position.rank as u8 + 2).unwrap(),
                        }),
                    };
                    if let Some(position_in_front) = position_in_front {
                        let troop_in_front = &self.get_square(&position_in_front).troop;
                        match troop_in_front {
                            Some(troop_in_front) => {
                                if troop_in_front.color != troop.color {
                                    valid_moves.push(self.get_square(&position_in_front));
                                }
                            }
                            None => {
                                valid_moves.push(self.get_square(&position_in_front));
                            }
                        }
                    }
                    if troop.position.rank == Rank::Two {
                        let position_two_in_front = Position {
                            file: troop.position.file,
                            rank: Rank::try_from(troop.position.rank as u8 + 3).unwrap(),
                        };
                        let troop_two_in_front =
                            self.get_square(&position_two_in_front).troop.as_ref();
                        if troop_two_in_front.is_none()
                            && self.get_square(&position_in_front.unwrap()).troop.is_none()
                        {
                            valid_moves.push(self.get_square(&position_two_in_front));
                        }
                    }
                    let position_diagonal_left = match troop.position.file {
                        File::A => None,
                        _ => Some(Position {
                            file: File::try_from(troop.position.file as u8).unwrap(),
                            rank: Rank::try_from(troop.position.rank as u8 + 2).unwrap(),
                        }),
                    };
                    if let Some(position_diagonal_left) = position_diagonal_left {
                        let troop_diagonal_left =
                            self.get_square(&position_diagonal_left).troop.as_ref();
                        if let Some(troop_diagonal_left) = troop_diagonal_left {
                            if troop_diagonal_left.color != troop.color {
                                valid_moves.push(self.get_square(&position_diagonal_left));
                            }
                        }
                    }
                    let position_diagonal_right = match troop.position.file {
                        File::H => None,
                        _ => Some(Position {
                            file: File::try_from(troop.position.file as u8 + 2).unwrap(),
                            rank: Rank::try_from(troop.position.rank as u8 + 2).unwrap(),
                        }),
                    };
                    if let Some(position_diagonal_right) = position_diagonal_right {
                        let troop_diagonal_right =
                            self.get_square(&position_diagonal_right).troop.as_ref();
                        if let Some(troop_diagonal_right) = troop_diagonal_right {
                            if troop_diagonal_right.color != troop.color {
                                valid_moves.push(self.get_square(&position_diagonal_right));
                            }
                        }
                    }
                }
                Color::Black => {
                    let position_in_front = match troop.position.rank {
                        Rank::One => None,
                        _ => Some(Position {
                            file: troop.position.file,
                            rank: Rank::try_from(troop.position.rank as u8).unwrap(),
                        }),
                    };
                    if let Some(position_in_front) = position_in_front {
                        let troop_in_front = &self.get_square(&position_in_front).troop;
                        match troop_in_front {
                            Some(troop_in_front) => {
                                if troop_in_front.color != troop.color {
                                    valid_moves.push(self.get_square(&position_in_front));
                                }
                            }
                            None => {
                                valid_moves.push(self.get_square(&position_in_front));
                            }
                        }
                    }
                    if troop.position.rank == Rank::Seven {
                        let position_two_in_front = Position {
                            file: troop.position.file,
                            rank: Rank::try_from(troop.position.rank as u8 - 1).unwrap(),
                        };
                        let troop_two_in_front =
                            self.get_square(&position_two_in_front).troop.as_ref();
                        if troop_two_in_front.is_none()
                            && self.get_square(&position_in_front.unwrap()).troop.is_none()
                        {
                            valid_moves.push(self.get_square(&position_two_in_front));
                        }
                    }
                    let position_diagonal_left = match troop.position.file {
                        File::A => None,
                        _ => Some(Position {
                            file: File::try_from(troop.position.file as u8).unwrap(),
                            rank: Rank::try_from(troop.position.rank as u8).unwrap(),
                        }),
                    };
                    if let Some(position_diagonal_left) = position_diagonal_left {
                        let troop_diagonal_left =
                            self.get_square(&position_diagonal_left).troop.as_ref();
                        if let Some(troop_diagonal_left) = troop_diagonal_left {
                            if troop_diagonal_left.color != troop.color {
                                valid_moves.push(self.get_square(&position_diagonal_left));
                            }
                        }
                    }
                    let position_diagonal_right = match troop.position.file {
                        File::H => None,
                        _ => Some(Position {
                            file: File::try_from(troop.position.file as u8 + 2).unwrap(),
                            rank: Rank::try_from(troop.position.rank as u8).unwrap(),
                        }),
                    };
                    if let Some(position_diagonal_right) = position_diagonal_right {
                        let troop_diagonal_right =
                            self.get_square(&position_diagonal_right).troop.as_ref();
                        if let Some(troop_diagonal_right) = troop_diagonal_right {
                            if troop_diagonal_right.color != troop.color {
                                valid_moves.push(self.get_square(&position_diagonal_right));
                            }
                        }
                    }
                }
            },
            Piece::Rook => {
                let mut rank_num = troop.position.rank as u8;
                while let Ok(rank) = Rank::try_from(rank_num) {
                    let blocking_troop = self
                        .get_square(&Position {
                            file: troop.position.file,
                            rank,
                        })
                        .troop
                        .as_ref();
                    match blocking_troop {
                        Some(blocking_troop) => {
                            if blocking_troop.color != troop.color {
                                valid_moves.push(self.get_square(&blocking_troop.position));
                            }
                            break;
                        }
                        None => {
                            valid_moves.push(self.get_square(&Position {
                                file: troop.position.file,
                                rank,
                            }));
                        }
                    }
                    rank_num -= 1;
                }
                rank_num = troop.position.rank as u8 + 2;
                while let Ok(rank) = Rank::try_from(rank_num) {
                    let blocking_troop = self
                        .get_square(&Position {
                            file: troop.position.file,
                            rank,
                        })
                        .troop
                        .as_ref();
                    match blocking_troop {
                        Some(blocking_troop) => {
                            if blocking_troop.color != troop.color {
                                valid_moves.push(self.get_square(&blocking_troop.position));
                            }
                            break;
                        }
                        None => {
                            valid_moves.push(self.get_square(&Position {
                                file: troop.position.file,
                                rank,
                            }));
                        }
                    }
                    rank_num += 1;
                }
                let mut file_num = troop.position.file as u8;
                while let Ok(file) = File::try_from(file_num) {
                    let blocking_troop = self
                        .get_square(&Position {
                            rank: troop.position.rank,
                            file,
                        })
                        .troop
                        .as_ref();
                    match blocking_troop {
                        Some(blocking_troop) => {
                            if blocking_troop.color != troop.color {
                                valid_moves.push(self.get_square(&blocking_troop.position));
                            }
                            break;
                        }
                        None => {
                            valid_moves.push(self.get_square(&Position {
                                rank: troop.position.rank,
                                file,
                            }));
                        }
                    }
                    file_num -= 1;
                }
                file_num = troop.position.file as u8 + 2;
                while let Ok(file) = File::try_from(file_num) {
                    let blocking_troop = self
                        .get_square(&Position {
                            rank: troop.position.rank,
                            file,
                        })
                        .troop
                        .as_ref();
                    match blocking_troop {
                        Some(blocking_troop) => {
                            if blocking_troop.color != troop.color {
                                valid_moves.push(self.get_square(&blocking_troop.position));
                            }
                            break;
                        }
                        None => {
                            valid_moves.push(self.get_square(&Position {
                                rank: troop.position.rank,
                                file,
                            }));
                        }
                    }
                    file_num += 1;
                }
            }
            Piece::Knight => {
                let new_rank = Rank::try_from(troop.position.rank as u8 + 2);
                let new_file = match troop.position.file {
                    File::A => Err(Error::FileParse),
                    _ => File::try_from(troop.position.file as u8 - 1),
                };
                if let (Ok(file), Ok(rank)) = (new_file, new_rank) {
                    match self.get_square(&Position { file, rank }).troop.as_ref() {
                        Some(blocking_troop) => {
                            if blocking_troop.color != troop.color {
                                valid_moves.push(self.get_square(&blocking_troop.position));
                            }
                        }
                        None => valid_moves.push(self.get_square(&Position { file, rank })),
                    }
                }
                let new_rank = Rank::try_from(troop.position.rank as u8 + 3);
                let new_file = File::try_from(troop.position.file as u8);
                if let (Ok(file), Ok(rank)) = (new_file, new_rank) {
                    match self.get_square(&Position { file, rank }).troop.as_ref() {
                        Some(blocking_troop) => {
                            if blocking_troop.color != troop.color {
                                valid_moves.push(self.get_square(&blocking_troop.position));
                            }
                        }
                        None => valid_moves.push(self.get_square(&Position { file, rank })),
                    }
                }
                let new_rank = Rank::try_from(troop.position.rank as u8 + 3);
                let new_file = File::try_from(troop.position.file as u8 + 2);
                if let (Ok(file), Ok(rank)) = (new_file, new_rank) {
                    match self.get_square(&Position { file, rank }).troop.as_ref() {
                        Some(blocking_troop) => {
                            if blocking_troop.color != troop.color {
                                valid_moves.push(self.get_square(&blocking_troop.position));
                            }
                        }
                        None => valid_moves.push(self.get_square(&Position { file, rank })),
                    }
                }
                let new_rank = Rank::try_from(troop.position.rank as u8 + 2);
                let new_file = File::try_from(troop.position.file as u8 + 3);
                if let (Ok(file), Ok(rank)) = (new_file, new_rank) {
                    match self.get_square(&Position { file, rank }).troop.as_ref() {
                        Some(blocking_troop) => {
                            if blocking_troop.color != troop.color {
                                valid_moves.push(self.get_square(&blocking_troop.position));
                            }
                        }
                        None => valid_moves.push(self.get_square(&Position { file, rank })),
                    }
                }
                let new_rank = Rank::try_from(troop.position.rank as u8);
                let new_file = File::try_from(troop.position.file as u8 + 3);
                if let (Ok(file), Ok(rank)) = (new_file, new_rank) {
                    match self.get_square(&Position { file, rank }).troop.as_ref() {
                        Some(blocking_troop) => {
                            if blocking_troop.color != troop.color {
                                valid_moves.push(self.get_square(&blocking_troop.position));
                            }
                        }
                        None => valid_moves.push(self.get_square(&Position { file, rank })),
                    }
                }
                let new_rank = match troop.position.rank {
                    Rank::One => Err(Error::RankParse),
                    _ => Rank::try_from(troop.position.rank as u8 - 1),
                };
                let new_file = File::try_from(troop.position.file as u8 + 2);
                if let (Ok(file), Ok(rank)) = (new_file, new_rank) {
                    match self.get_square(&Position { file, rank }).troop.as_ref() {
                        Some(blocking_troop) => {
                            if blocking_troop.color != troop.color {
                                valid_moves.push(self.get_square(&blocking_troop.position));
                            }
                        }
                        None => valid_moves.push(self.get_square(&Position { file, rank })),
                    }
                }
                let new_rank = match troop.position.rank {
                    Rank::One => Err(Error::RankParse),
                    _ => Rank::try_from(troop.position.rank as u8 - 1),
                };
                let new_file = File::try_from(troop.position.file as u8);
                if let (Ok(file), Ok(rank)) = (new_file, new_rank) {
                    match self.get_square(&Position { file, rank }).troop.as_ref() {
                        Some(blocking_troop) => {
                            if blocking_troop.color != troop.color {
                                valid_moves.push(self.get_square(&blocking_troop.position));
                            }
                        }
                        None => valid_moves.push(self.get_square(&Position { file, rank })),
                    }
                }
                let new_rank = Rank::try_from(troop.position.rank as u8);
                let new_file = match troop.position.file {
                    File::A => Err(Error::FileParse),
                    _ => File::try_from(troop.position.file as u8 - 1),
                };
                if let (Ok(file), Ok(rank)) = (new_file, new_rank) {
                    match self.get_square(&Position { file, rank }).troop.as_ref() {
                        Some(blocking_troop) => {
                            if blocking_troop.color != troop.color {
                                valid_moves.push(self.get_square(&blocking_troop.position));
                            }
                        }
                        None => valid_moves.push(self.get_square(&Position { file, rank })),
                    }
                }
            }
            _ => todo!(),
        }

        valid_moves
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    #[cfg(any(test, debug_assertions))]
    pub fn remove_troop(&mut self, position: Position) -> Option<Troop> {
        let square = self
            .squares
            .get_mut(&position.file)
            .unwrap()
            .get_mut(&position.rank)
            .unwrap();
        square.troop.take()
    }

    #[cfg(any(test, debug_assertions))]
    pub fn place_troop(&mut self, troop: Troop) -> Result<(), SquareOccupied> {
        if self.get_square(&troop.position).troop.is_some() {
            return Err(SquareOccupied);
        }
        self.get_mut_square(&troop.position).troop = Some(troop.clone());
        Ok(())
    }

    #[cfg(any(test, debug_assertions))]
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

    #[cfg(any(test, debug_assertions))]
    pub fn set_state(&mut self, state: BoardState) {
        self.state = state;
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some((*self as u8).cmp(&(*other as u8)))
    }
}
impl Ord for Rank {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as u8).cmp(&(*other as u8))
    }
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
            _ => {
                eprintln!("Invalid rank: {}", rank);
                Err(Error::RankParse)
            }
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
impl PartialOrd for File {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some((*self as u8).cmp(&(*other as u8)))
    }
}
impl Ord for File {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (*self as u8).cmp(&(*other as u8))
    }
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
            _ => {
                eprintln!("Invalid file: {}", file);
                Err(Error::FileParse)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Position {
    pub file: File,
    pub rank: Rank,
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Square {
    pub troop: Option<Troop>,
    pub position: Position,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Piece {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Troop {
    pub piece: Piece,
    pub color: Color,
    pub position: Position,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Color {
    Black,
    White,
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize,))]
pub enum BoardState {
    ToMove(Color),
    Check(Color),
    Checkmate(Color),
    Stalemate,
    Draw,
}
#[cfg(feature = "actions")]
impl BoardState {
    fn can_move(&self, team: Color) -> bool {
        match self {
            BoardState::ToMove(color) => *color == team,
            BoardState::Check(color) => *color == team,
            _ => false,
        }
    }
}
