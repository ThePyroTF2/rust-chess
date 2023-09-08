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
        Err(Error::Move(MoveError::InvalidPath(
            "King cannot move more than one space in any direction"
        ))),
    );
}
