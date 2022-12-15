use crate::shared::board::array_board::ArrayBoard;
use crate::shared::board::board_get_set::BoardGet;
use crate::shared::board::board_get_set::BoardSet;
use crate::shared::board::board_size::BoardSize;
use crate::shared::board::board_view::AsBoardView;
use crate::shared::board::board_view::BoardView;

use super::cell::Connect4Cell;
use super::symbol::Symbol;

pub struct Connect4<const WIDTH: usize, const HEIGHT: usize> {
    /// How many tokens in a row do you need to win.
    pub winning_tokens: i32,

    /// The Connect4 game board.
    board: ArrayBoard<WIDTH, HEIGHT, Connect4Cell>,
}

impl<const WIDTH: usize, const HEIGHT: usize> Connect4<WIDTH, HEIGHT> {
    pub fn new(winning_tokens: i32) -> Self {
        Self {
            winning_tokens,
            ..Default::default()
        }
    }

    pub fn view_as_xo(
        &self,
    ) -> BoardView<ArrayBoard<WIDTH, HEIGHT, Connect4Cell>, impl Fn(&Connect4Cell) -> &char> {
        self.board.as_view(|cell| match cell.symbol {
            Symbol::Empty => &' ',
            Symbol::Red => &'X',
            Symbol::Yellow => &'O',
        })
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Default for Connect4<WIDTH, HEIGHT> {
    fn default() -> Self {
        Self {
            winning_tokens: Default::default(),
            board: ArrayBoard::<WIDTH, HEIGHT, Connect4Cell>::generate(|_point| Default::default()),
        }
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> BoardSize for Connect4<WIDTH, HEIGHT> {
    fn width(&self) -> i32 {
        self.board.width()
    }

    fn height(&self) -> i32 {
        self.board.height()
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> BoardGet for Connect4<WIDTH, HEIGHT> {
    type Value = Connect4Cell;
    fn get(&self, point: &crate::shared::coord::point::Point) -> &Connect4Cell {
        self.board.get(point)
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> BoardSet for Connect4<WIDTH, HEIGHT> {
    type Value = Connect4Cell;
    fn get_mut(&mut self, point: &crate::shared::coord::point::Point) -> &mut Connect4Cell {
        self.board.get_mut(point)
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> std::fmt::Display for Connect4<WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.board.fmt(f)
    }
}
