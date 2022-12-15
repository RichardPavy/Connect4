use crate::shared::board::board_size::BoardSize;

use super::board::Connect4;
use super::symbol::Symbol;

impl<const WIDTH: usize, const HEIGHT: usize> Connect4<WIDTH, HEIGHT> {
    pub fn next_move(&mut self, symbol: Symbol, max_depth: i32) -> Option<Move> {
        let next_move = self.next_move_impl(symbol, 0, max_depth);
        if let Some(next_move) = next_move.as_ref() {
            *self.get_dropped_pos_mut(next_move.column).unwrap().symbol = symbol;
        }
        next_move
    }

    fn next_move_impl(&mut self, symbol: Symbol, score: i32, max_depth: i32) -> Option<Move> {
        let mut best_move: Option<Move> = None;
        for column in 0..self.width() {
            if let Some(play_column) = self.play_column(symbol, column) {
                if play_column.end_of_game || max_depth == 0 {
                    Self::update_best_move(
                        &mut best_move,
                        column,
                        score + play_column.delta_score,
                        play_column.end_of_game,
                    );
                } else if let Some(opponent_play) = self.next_move_impl(
                    symbol.other(),
                    score - play_column.delta_score,
                    max_depth - 1,
                ) {
                    Self::update_best_move(
                        &mut best_move,
                        column,
                        score - opponent_play.score,
                        play_column.end_of_game,
                    );
                }
                play_column.undo(self);
            };
        }
        best_move
    }

    fn update_best_move(best_move: &mut Option<Move>, column: i32, score: i32, end_of_game: bool) {
        if if let Some(best_move) = best_move {
            best_move.score < score
        } else {
            true
        } {
            *best_move = Some(Move {
                column,
                score,
                end_of_game,
            })
        }
    }
}

#[derive(Debug)]
pub struct Move {
    pub column: i32,
    pub score: i32,
    pub end_of_game: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_move() {
        let mut board = Connect4::<5, 4>::new(3);
        let depth = 1;

        board.play_column(Symbol::Yellow, 0);
        board.play_column(Symbol::Yellow, 0);
        board.next_move(Symbol::Red, depth);
        board.next_move(Symbol::Red, depth);
        board.play_column(Symbol::Yellow, 1);
        board.next_move(Symbol::Red, depth);
        println!("{}", board);
        assert_eq!(
            r#"
    | 1 | 2 | 3 | 4 | 5 
----+---+---+---+---+---
 A  |   |   |   |   |   
----+---+---+---+---+---
 B  | R |   |   |   |   
----+---+---+---+---+---
 C  | Y | R |   |   |   
----+---+---+---+---+---
 D  | Y | Y | R |   |   
"#,
            format!("\n{board}")
        );
    }
}
