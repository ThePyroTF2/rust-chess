use crate::*;

#[test]
fn standard_movement() {
    let mut board = Board::default();
    board.remove_troop(Position {
        file: File::E,
        rank: Rank::Two,
    });
    assert_eq!(
        board.move_troop(
            Position {
                file: File::E,
                rank: Rank::One
            },
            Position {
                file: File::E,
                rank: Rank::Two
            },
        ),
        Ok(()),
    );
}

#[test]
fn invalid_movement() {
    let mut board = Board::default();
    assert_eq!(
        board.move_troop(
            Position {
                file: File::E,
                rank: Rank::One
            },
            Position {
                file: File::E,
                rank: Rank::Three
            },
        ),
        Err(Error::Move(MoveError::Other)),
    );
}

#[test]
fn move_into_check() {
    let mut board = Board::default();
    board.remove_troop(Position {
        file: File::C,
        rank: Rank::Seven,
    });
    board.remove_troop(Position {
        file: File::F,
        rank: Rank::Two,
    });
    board.set_state(BoardState::ToMove(Color::Black));
    board
        .move_troop(
            Position {
                file: File::D,
                rank: Rank::Eight,
            },
            Position {
                file: File::B,
                rank: Rank::Six,
            },
        )
        .unwrap();
    assert_eq!(
        board.move_troop(
            Position {
                file: File::E,
                rank: Rank::One
            },
            Position {
                file: File::F,
                rank: Rank::Two
            },
        ),
        Err(Error::Move(MoveError::Other)),
    );
}
