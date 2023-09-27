use crate::*;

pub mod check;
pub mod movement;

// #[test]
// fn checkmate() {
//     let mut board = Board::default();
//     board
//         .move_troop(
//             Position {
//                 file: File::F,
//                 rank: Rank::Two,
//             },
//             Position {
//                 file: File::F,
//                 rank: Rank::Three,
//             },
//         )
//         .unwrap();
//     board
//         .move_troop(
//             Position {
//                 file: File::E,
//                 rank: Rank::Seven,
//             },
//             Position {
//                 file: File::E,
//                 rank: Rank::Six,
//             },
//         )
//         .unwrap();
//     board
//         .move_troop(
//             Position {
//                 file: File::G,
//                 rank: Rank::Two,
//             },
//             Position {
//                 file: File::G,
//                 rank: Rank::Four,
//             },
//         )
//         .unwrap();
//     board
//         .move_troop(
//             Position {
//                 file: File::D,
//                 rank: Rank::Eight,
//             },
//             Position {
//                 file: File::H,
//                 rank: Rank::Four,
//             },
//         )
//         .unwrap();
//
//     assert_eq!(board.state, BoardState::Checkmate(Color::White));
// }
