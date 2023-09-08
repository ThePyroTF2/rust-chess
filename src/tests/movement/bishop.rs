use crate::*;

#[test]
fn standard_movement() {
    let mut board = Board::default();
    board.remove_troop(Position {
        file: File::D,
        rank: Rank::Two,
    });
    assert_eq!(
        board.move_troop(
            Position {
                file: File::C,
                rank: Rank::One,
            },
            Position {
                file: File::F,
                rank: Rank::Four,
            },
        ),
        Ok(())
    );
}

#[test]
fn invalid_movement() {
    let mut board = Board::default();
    assert_eq!(
        board.move_troop(
            Position {
                file: File::C,
                rank: Rank::One,
            },
            Position {
                file: File::C,
                rank: Rank::Four,
            },
        ),
        Err(Error::Move(MoveError::InvalidPath(
            "Bishop must move in a purely diagonal line"
        )))
    );
}

#[test]
fn blocked_path() {
    let mut board = Board::default();
    assert_eq!(
        board.move_troop(
            Position {
                file: File::C,
                rank: Rank::One
            },
            Position {
                file: File::E,
                rank: Rank::Three
            }
        ),
        Err(Error::Move(MoveError::PathIsBlocked))
    );
}
