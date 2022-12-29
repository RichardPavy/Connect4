use lazy_static::lazy_static;

use crate::shared::board::board_get_set::BoardGet;
use crate::shared::board::board_get_set::BoardSet;
use crate::shared::board::board_lines::BoardLines;
use crate::shared::board::board_size::BoardSize;
use crate::shared::coord::directions;
use crate::shared::coord::point::Point;

use super::board::Connect4;
use super::cell::Connect4Cell;
use super::symbol::Symbol;

impl<const WIDTH: usize, const HEIGHT: usize> Connect4<WIDTH, HEIGHT> {
    pub fn play_column(&mut self, symbol: Symbol, column: i32) -> Option<PlayColumn> {
        if let Some(dropped_pos) = self.get_dropped_pos_mut(column) {
            *dropped_pos.symbol = symbol;
            let position = dropped_pos.position;
            let (delta_score, end_of_game) = {
                let my_score = self.eval_position(symbol, position);
                let other_score = self.eval_position(symbol.other(), position);
                let delta_score = my_score.score - other_score.score;
                let end_of_game = my_score.end_of_game || other_score.end_of_game;
                (delta_score, end_of_game)
            };
            Some(PlayColumn {
                delta_score,
                end_of_game,
                position,
            })
        } else {
            None
        }
    }

    #[cfg(test)]
    pub fn get_dropped_pos(&self, column: i32) -> Option<DroppedPos> {
        use crate::shared::board::board_iterate::ro::BoardIterate;
        self.iterate(Point::new(column, self.height() - 1), directions::DOWN)
            .filter(|(_, cell)| cell.symbol == Symbol::Empty)
            .next()
            .map(|(position, cell)| DroppedPos {
                position,
                symbol: &cell.symbol,
            })
    }

    pub fn get_dropped_pos_mut(&mut self, column: i32) -> Option<DroppedPosMut> {
        use crate::shared::board::board_iterate::rw::BoardIterateMut;
        self.iterate_mut(Point::new(column, self.height() - 1), directions::DOWN)
            .filter(|(_, cell)| cell.symbol == Symbol::Empty)
            .next()
            .map(|(position, cell)| DroppedPosMut {
                position,
                symbol: &mut cell.symbol,
            })
    }

    fn eval_position(&self, symbol: Symbol, position: Point) -> Score {
        let mut end_of_game = false;
        let score = self
            .get_intersecting_lines(position)
            .map(|line| {
                let mut matches = 0;
                let mut score = 1;
                for curr_pos in line {
                    let curr_symbol = self.get(&curr_pos).symbol;
                    if curr_symbol == Symbol::Empty {
                        score *= 10
                    } else if curr_symbol == symbol {
                        score *= 100;
                        matches += 1;
                    } else {
                        return 0;
                    }
                }
                if matches == self.winning_tokens {
                    end_of_game = true
                }
                return score;
            })
            .sum();
        Score { score, end_of_game }
    }

    fn get_intersecting_lines<'a>(
        &'a self,
        position: Point,
    ) -> impl Iterator<Item = impl Iterator<Item = Point>> + 'a {
        lazy_static! {
            // TODO; Create a proc macro where '+' operator is re-written as call to 'a.plus(b) that can be const'
            static ref DIRECTIONS: [Point; 4] = [
                directions::UP,
                directions::RIGHT,
                directions::UP + directions::RIGHT,
                directions::UP + directions::LEFT,
            ];
        }
        DIRECTIONS
            .iter()
            .map(move |direction| self.lines(position, *direction, self.winning_tokens))
            .flatten()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PlayColumn {
    pub delta_score: i32,
    pub end_of_game: bool,
    pub position: Point,
}

impl PlayColumn {
    pub fn undo<B: BoardSet<Value = Connect4Cell>>(&self, board: &mut B) {
        let cell = board.get_mut(&self.position);
        debug_assert!(cell.symbol != Symbol::Empty);
        cell.symbol = Symbol::Empty;
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Score {
    score: i32,
    end_of_game: bool,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DroppedPos<'board> {
    position: Point,
    symbol: &'board Symbol,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DroppedPosMut<'board> {
    pub position: Point,
    pub symbol: &'board mut Symbol,
}

#[cfg(test)]
mod tests {
    use crate::shared::board::board_size::BoardSize;

    use super::*;

    #[test]
    fn get_dropped_pos() {
        let mut board = Connect4::<5, 4>::new(3);

        let pos = board
            .get_dropped_pos(2)
            .map(|dropped_pos| dropped_pos.position);
        assert_eq!(Some(Point::new(2, 3)), pos);

        let mut symbol = Symbol::Red;
        for j in (1..board.height()).rev() {
            let pos = Point::new(2, j);
            board.set(&pos, Connect4Cell { symbol });

            let pos = board
                .get_dropped_pos(2)
                .map(|dropped_pos| dropped_pos.position);
            assert_eq!(Some(Point::new(2, j - 1)), pos);
            symbol = symbol.other();
        }

        println!("{}", board);
        assert_eq!(
            r#"
    | 1 | 2 | 3 | 4 | 5 
----+---+---+---+---+---
 A  |   |   |   |   |   
----+---+---+---+---+---
 B  |   |   | R |   |   
----+---+---+---+---+---
 C  |   |   | Y |   |   
----+---+---+---+---+---
 D  |   |   | R |   |   
"#,
            format!("\n{board}")
        );

        board.set(&Point::new(2, 0), Connect4Cell { symbol });

        let dropped_pos = board.get_dropped_pos(2);
        assert_eq!(None, dropped_pos);
    }

    #[test]
    fn get_dropped_pos_mut() {
        let mut board = Connect4::<5, 4>::new(3);
        for i in 0..board.width() {
            let mut symbol = Symbol::Red;
            for j in 0..i {
                *board.get_dropped_pos_mut(j).unwrap().symbol = symbol;
                symbol = symbol.other();
            }
        }

        println!("{}", board);
        assert_eq!(
            r#"
    | 1 | 2 | 3 | 4 | 5 
----+---+---+---+---+---
 A  | R |   |   |   |   
----+---+---+---+---+---
 B  | R | Y |   |   |   
----+---+---+---+---+---
 C  | R | Y | R |   |   
----+---+---+---+---+---
 D  | R | Y | R | Y |   
"#,
            format!("\n{board}")
        );
    }

    #[test]
    fn get_intersecting_lines_0_0() {
        let mut board = Connect4::<5, 4>::new(3);
        for pos in board
            .get_intersecting_lines(Point::new(0, 0))
            .flatten()
            .collect::<Vec<Point>>()
        {
            board.get_mut(&pos).symbol = Symbol::Red;
        }
        println!("{}", board);
        assert_eq!(
            r#"
    | 1 | 2 | 3 | 4 | 5 
----+---+---+---+---+---
 A  | R | R | R |   |   
----+---+---+---+---+---
 B  | R | R |   |   |   
----+---+---+---+---+---
 C  | R |   | R |   |   
----+---+---+---+---+---
 D  |   |   |   |   |   
"#,
            format!("\n{board}")
        );
    }

    #[test]
    fn get_intersecting_lines_center() {
        let mut board = Connect4::<7, 7>::new(3);
        for pos in board
            .get_intersecting_lines(Point::new(3, 3))
            .flatten()
            .collect::<Vec<Point>>()
        {
            board.get_mut(&pos).symbol = Symbol::Red
        }
        println!("{}", board);
        assert_eq!(
            r#"
    | 1 | 2 | 3 | 4 | 5 | 6 | 7 
----+---+---+---+---+---+---+---
 A  |   |   |   |   |   |   |   
----+---+---+---+---+---+---+---
 B  |   | R |   | R |   | R |   
----+---+---+---+---+---+---+---
 C  |   |   | R | R | R |   |   
----+---+---+---+---+---+---+---
 D  |   | R | R | R | R | R |   
----+---+---+---+---+---+---+---
 E  |   |   | R | R | R |   |   
----+---+---+---+---+---+---+---
 F  |   | R |   | R |   | R |   
----+---+---+---+---+---+---+---
 G  |   |   |   |   |   |   |   
"#,
            format!("\n{board}")
        );
    }

    #[test]
    fn eval_position() {
        let mut board = Connect4::<5, 4>::new(3);

        let pos2d = board
            .get_dropped_pos(1)
            .map(|dropped_pos| dropped_pos.position)
            .unwrap();
        assert_eq!(4000, board.eval_position(Symbol::Red, pos2d).score);
        assert_eq!(4000, board.eval_position(Symbol::Yellow, pos2d).score);

        board
            .get_dropped_pos_mut(1)
            .map(|dropped_pos| *dropped_pos.symbol = Symbol::Red);
        assert_eq!(40000, board.eval_position(Symbol::Red, pos2d).score);
        assert_eq!(0, board.eval_position(Symbol::Yellow, pos2d).score);

        let pos1d = board
            .get_dropped_pos(0)
            .map(|dropped_pos| dropped_pos.position)
            .unwrap();

        assert_eq!(12000, board.eval_position(Symbol::Red, pos1d).score);
        assert_eq!(2000, board.eval_position(Symbol::Yellow, pos1d).score);

        board
            .get_dropped_pos_mut(1)
            .map(|dropped_pos| *dropped_pos.symbol = Symbol::Red);
        board
            .get_dropped_pos_mut(0)
            .map(|dropped_pos| *dropped_pos.symbol = Symbol::Yellow);
        board
            .get_dropped_pos_mut(0)
            .map(|dropped_pos| *dropped_pos.symbol = Symbol::Red);

        let pos1b = board
            .get_dropped_pos(0)
            .map(|dropped_pos| dropped_pos.position)
            .unwrap();
        assert_eq!(21000, board.eval_position(Symbol::Red, pos1b).score);
        assert_eq!(1000, board.eval_position(Symbol::Yellow, pos1b).score);
        assert_eq!(0, board.eval_position(Symbol::Yellow, pos1d).score);

        let pos2b = board
            .get_dropped_pos_mut(1)
            .map(|dropped_pos| {
                *dropped_pos.symbol = Symbol::Red;
                dropped_pos.position
            })
            .unwrap();
        assert_eq!(
            Score {
                score: 1240000,
                end_of_game: true
            },
            board.eval_position(Symbol::Red, pos2b)
        );
        assert_eq!(
            Score {
                score: 0,
                end_of_game: false
            },
            board.eval_position(Symbol::Yellow, pos2b)
        );

        println!("{}", board);
    }
}
