use crate::*;

#[test]
fn standard_movement() {
    let mut board = Board::default();
    assert_eq!(
        board.move_troop(
            Position {
                file: File::A,
                rank: Rank::Two,
            },
            Position {
                file: File::A,
                rank: Rank::Three,
            }
        ),
        Ok(())
    );
}

#[test]
fn double_move_white() {
    let mut board = Board::default();
    assert_eq!(
        board.move_troop(
            Position {
                file: File::A,
                rank: Rank::Two,
            },
            Position {
                file: File::A,
                rank: Rank::Four,
            }
        ),
        Ok(())
    );
}

#[test]
fn double_move_black() {
    let mut board = Board::default();
    board.set_state(BoardState::ToMove(Color::Black));
    assert_eq!(
        board.move_troop(
            Position {
                file: File::A,
                rank: Rank::Seven,
            },
            Position {
                file: File::A,
                rank: Rank::Five,
            }
        ),
        Ok(())
    );
}

#[test]
fn capture() {
    let mut board = Board::default();
    board
        .place_troop(Troop {
            color: Color::Black,
            piece: Piece::Pawn,
            position: Position {
                file: File::B,
                rank: Rank::Three,
            },
        })
        .unwrap();
    assert_eq!(
        board.move_troop(
            Position {
                file: File::A,
                rank: Rank::Two,
            },
            Position {
                file: File::B,
                rank: Rank::Three,
            }
        ),
        Ok(())
    );
}

#[test]
fn non_capture_diagonal() {
    let mut board = Board::default();
    assert_eq!(
        board.move_troop(
            Position {
                file: File::A,
                rank: Rank::Two,
            },
            Position {
                file: File::B,
                rank: Rank::Three,
            }
        ),
        Err(Error::Move(MoveError::Other))
    );
}

#[test]
fn two_squares_horizontally() {
    let mut board = Board::default();
    assert_eq!(
        board.move_troop(
            Position {
                file: File::A,
                rank: Rank::Two,
            },
            Position {
                file: File::C,
                rank: Rank::Three,
            }
        ),
        Err(Error::Move(MoveError::Other))
    );
}

#[test]
fn three_squares_vertically() {
    let mut board = Board::default();
    assert_eq!(
        board.move_troop(
            Position {
                file: File::A,
                rank: Rank::Two,
            },
            Position {
                file: File::A,
                rank: Rank::Five,
            }
        ),
        Err(Error::Move(MoveError::Other))
    );
}

#[test]
fn blocked_path() {
    let mut board = Board::default();
    board
        .place_troop(Troop {
            piece: Piece::Pawn,
            color: Color::White,
            position: Position {
                file: File::A,
                rank: Rank::Three,
            },
        })
        .unwrap();

    assert_eq!(
        board.move_troop(
            Position {
                file: File::A,
                rank: Rank::Two
            },
            Position {
                file: File::A,
                rank: Rank::Four
            }
        ),
        Err(Error::Move(MoveError::Other))
    );
}

#[test]
fn backwards() {
    let mut board = Board::default();
    board
        .move_troop(
            Position {
                file: File::A,
                rank: Rank::Two,
            },
            Position {
                file: File::A,
                rank: Rank::Three,
            },
        )
        .unwrap();

    board.set_state(BoardState::ToMove(Color::White));

    assert_eq!(
        board.move_troop(
            Position {
                file: File::A,
                rank: Rank::Three
            },
            Position {
                file: File::A,
                rank: Rank::Two
            }
        ),
        Err(Error::Move(MoveError::Other))
    );
}
