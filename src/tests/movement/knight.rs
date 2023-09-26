use crate::*;

#[test]
fn standard_movement() {
    let mut board = Board::default();
    assert_eq!(
        board.move_troop(
            Position {
                file: File::B,
                rank: Rank::One,
            },
            Position {
                file: File::C,
                rank: Rank::Three,
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
                file: File::B,
                rank: Rank::One,
            },
            Position {
                file: File::C,
                rank: Rank::Four,
            },
        ),
        Err(Error::Move(MoveError::Other)),
    );
}
