use crate::*;

#[test]
fn direct_check() {
    let mut board = Board::default();
    board.remove_troop(Position {
        file: File::F,
        rank: Rank::Two,
    });
    board.remove_troop(Position {
        file: File::C,
        rank: Rank::Seven,
    });
    board
        .move_troop(
            Position {
                file: File::E,
                rank: Rank::One,
            },
            Position {
                file: File::F,
                rank: Rank::Two,
            },
        )
        .unwrap();
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

    assert_eq!(board.state, BoardState::Check(Color::White));
}

#[test]
fn indirect_check() {
    let mut board = Board::default();
    board
        .place_troop(Troop {
            color: Color::Black,
            piece: Piece::Bishop,
            position: Position {
                file: File::H,
                rank: Rank::Four,
            },
        })
        .unwrap();
    board
        .place_troop(Troop {
            color: Color::Black,
            piece: Piece::Pawn,
            position: Position {
                file: File::G,
                rank: Rank::Three,
            },
        })
        .unwrap();

    board
        .move_troop(
            Position {
                file: File::F,
                rank: Rank::Two,
            },
            Position {
                file: File::F,
                rank: Rank::Four,
            },
        )
        .unwrap();

    board
        .move_troop(
            Position {
                file: File::G,
                rank: Rank::Three,
            },
            Position {
                file: File::H,
                rank: Rank::Two,
            },
        )
        .unwrap();

    assert_eq!(board.state, BoardState::Check(Color::White));
}
