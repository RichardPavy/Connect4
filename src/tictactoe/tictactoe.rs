use lazy_static::lazy_static;

use crate::shared::board::board_get_set::BoardGet;
use crate::shared::board::board_get_set::BoardSet;
use crate::shared::board::board_iterate::ro::BoardIterate;
use crate::shared::board::board_size::BoardSize;
use crate::shared::board::board_to_string::AsChar;
use crate::shared::coord::directions;
use crate::shared::coord::point::Point;
use crate::tictactoe::board::Board;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Symbol {
    None,
    Circle,
    Cross,
}

impl Symbol {
    pub fn other(self) -> Symbol {
        match self {
            Symbol::None => Symbol::None,
            Symbol::Circle => Symbol::Cross,
            Symbol::Cross => Symbol::Circle,
        }
    }
}

impl AsChar for Symbol {
    fn as_char(&self) -> char {
        match self {
            Symbol::None => ' ',
            Symbol::Circle => 'O',
            Symbol::Cross => 'X',
        }
    }
}

lazy_static! {
    static ref LINES: Vec<[Point; 3]> = Board::generate(|_|Symbol::None).all_lines()
        .map(|line| line.collect::<Vec<Point>>())
        .filter(|line| line.len() == 3) // Filter out diagonals that don't go through one of the 4 corners.
        .map::<[Point; 3], _>(|line| line.try_into().unwrap())
        .collect();
}

pub trait TicTacToeBoard {
    fn eval(&self, symbol: Symbol) -> i32;
    fn next_move(&mut self, symbol: Symbol) -> Option<Game>;
    fn is_winning_move(&self, point: Point, symbol: Symbol) -> bool;
}

impl TicTacToeBoard for Board {
    fn eval(&self, symbol: Symbol) -> i32 {
        LINES
            .iter()
            .map(|line| {
                line.iter()
                    .map(|point| if *self.get(point) == symbol { 10 } else { 1 })
                    .fold(1, |acc, v| acc * v)
            })
            .max()
            .unwrap_or(0)
    }

    fn next_move(&mut self, symbol: Symbol) -> Option<Game> {
        let next_moves = self.all_points().filter_map(|point| {
            if *self.get(&point) != Symbol::None {
                return None;
            }

            let prev_symbol = *self.get(&point);
            *self.get_mut(&point) = symbol;

            let next_move = if self.is_winning_move(point, symbol)
                || self.is_winning_move(point, symbol.other())
            {
                Some(Game {
                    score: self.eval(symbol) - self.eval(symbol.other()),
                    symbol,
                    point,
                    next: None,
                })
            } else {
                if let Some(opponent_move) = self.next_move(symbol.other()) {
                    Some(Game {
                        score: -opponent_move.score,
                        symbol,
                        point,
                        next: Some(Box::new(opponent_move)),
                    })
                } else {
                    Some(Game {
                        score: 0,
                        symbol,
                        point,
                        next: None,
                    })
                }
            };

            *self.get_mut(&point) = prev_symbol;

            next_move
        });

        next_moves.max_by_key(|next_move| next_move.score)
    }

    fn is_winning_move(&self, point: Point, symbol: Symbol) -> bool {
        if self
            .iterate(Point { x: 0, ..point }, directions::RIGHT)
            .all(|(_point, value)| *value == symbol)
        {
            return true;
        }
        if self
            .iterate(Point { y: 0, ..point }, directions::UP)
            .all(|(_point, value)| *value == symbol)
        {
            return true;
        }
        if point.x == point.y {
            // First diagonal
            if self
                .iterate(Point::new(0, 0), directions::RIGHT + directions::UP)
                .all(|(_point, value)| *value == symbol)
            {
                return true;
            }
        }
        if point.x == self.height() - point.y - 1 {
            // Second diagonal
            if self
                .iterate(
                    Point::new(self.width() - 1, 0),
                    directions::LEFT + directions::UP,
                )
                .all(|(_point, value)| *value == symbol)
            {
                return true;
            }
        }
        return false;
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Game {
    score: i32,
    symbol: Symbol,
    point: Point,
    next: Option<Box<Game>>,
}

impl Game {
    pub fn score(&self) -> i32 {
        self.score
    }

    pub fn point(&self) -> Point {
        self.point
    }
}

#[cfg(test)]
mod tests {

    use crate::shared::coord::point::Point;

    use super::*;

    #[test]
    fn eval() {
        let mut board = Board::generate(|_| Symbol::None);
        assert_eq!(1, board.eval(Symbol::Circle));

        board.set(&Point::new(0, 0), Symbol::Circle);
        assert_eq!(10, board.eval(Symbol::Circle));

        board.set(&Point::new(1, 1), Symbol::Circle);
        assert_eq!(100, board.eval(Symbol::Circle));

        board.set(&Point::new(2, 2), Symbol::Circle);
        assert_eq!(1000, board.eval(Symbol::Circle));
    }
}
