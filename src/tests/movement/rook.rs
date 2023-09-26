use crate::*;

#[test]
fn standard_movement() {
    let mut board = Board::default();
    board.remove_troop(Position {
        file: File::A,
        rank: Rank::Two,
    });
    assert_eq!(
        board.move_troop(
            Position {
                file: File::A,
                rank: Rank::One,
            },
            Position {
                file: File::A,
                rank: Rank::Four,
            },
        ),
        Ok(()),
    );
}

#[test]
fn diagonal_movement() {
    let mut board = Board::default();
    assert_eq!(
        board.move_troop(
            Position {
                file: File::A,
                rank: Rank::One,
            },
            Position {
                file: File::C,
                rank: Rank::Three,
            },
        ),
        Err(Error::Move(MoveError::Other)),
    );
}

#[test]
fn blocked_vertical() {
    let mut board = Board::default();
    assert_eq!(
        board.move_troop(
            Position {
                file: File::A,
                rank: Rank::One
            },
            Position {
                file: File::A,
                rank: Rank::Three
            }
        ),
        Err(Error::Move(MoveError::Other))
    );
}

#[test]
fn blocked_horizonal() {
    let mut board = Board::default();
    board.remove_troop(Position {
        file: File::C,
        rank: Rank::One,
    });
    assert_eq!(
        board.move_troop(
            Position {
                file: File::A,
                rank: Rank::One
            },
            Position {
                file: File::C,
                rank: Rank::One
            }
        ),
        Err(Error::Move(MoveError::Other))
    );
}
