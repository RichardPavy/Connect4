use std::time;

use thousands::Separable;

use crate::puzzlesolver::puzzle_piece::PuzzlePiece;
use crate::puzzlesolver::solver::Puzzle;
use crate::shared::board::array_board::ArrayBoard;
use crate::shared::board::board_generate::BoardGenerate;

mod puzzle_piece;
mod shape;
mod solution;
mod solver;
mod solver_progress;
mod tagged_point;

pub fn solve_puzzle() {
    type Board = ArrayBoard<8, 8, PuzzlePiece>;
    let start = time::Instant::now();
    let sprites = [
        "
            O
            XO
             X
        ",
        "
            O
            X
            OXO
        ",
        "
            X
            OX
             OX
        ",
        "
             O
            OXOX
        ",
        "
            O
            XOX
              O
        ",
        "
            XOX
             XO
        ",
        "
            XOX
              OX
        ",
        "
            X
        ",
        "
            X
            OXO
            X
        ",
        "
            XO
            OX
        ",
        "
            OXOX
               O
        ",
        "
             O
            OXO
             O
        ",
        "
            X X
            OXO
        ",
        "
            XOXOX
        ",
    ];
    let mut board = Board::generate(|point| PuzzlePiece::blank(point));
    let solution = board.solve_puzzle(&sprites);
    println!("{}", solution);
    let end = time::Instant::now();
    println!(
        "{} micros",
        (end - start).as_micros().separate_with_spaces()
    );
}
