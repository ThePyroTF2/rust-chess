use crate::*;

#[test]
fn rook_movement() {
    let mut board = Board::default();
    board.remove_troop(Position {
        file: File::D,
        rank: Rank::Two,
    });
    assert_eq!(
        board.move_troop(
            Position {
                file: File::D,
                rank: Rank::One,
            },
            Position {
                file: File::D,
                rank: Rank::Four,
            },
        ),
        Ok(())
    );
}

#[test]
fn bishop_movement() {
    let mut board = Board::default();
    board.remove_troop(Position {
        file: File::E,
        rank: Rank::Two,
    });
    assert_eq!(
        board.move_troop(
            Position {
                file: File::D,
                rank: Rank::One,
            },
            Position {
                file: File::F,
                rank: Rank::Three,
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
                file: File::D,
                rank: Rank::One
            },
            Position {
                file: File::E,
                rank: Rank::Three
            }
        ),
        Err(Error::Move(MoveError::InvalidPath(
            "Queen must move in a purely vertical, horizontal, or diagonal line"
        )))
    );
}
