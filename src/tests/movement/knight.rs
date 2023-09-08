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
        Err(Error::Move(MoveError::InvalidPath(
            "Knight must move either two spaces horizontally and one space vertically, or two spaces vertically and one space horizontally"
        ))),
    );
}
