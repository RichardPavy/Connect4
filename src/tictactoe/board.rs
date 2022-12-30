use crate::shared::board::array_board::ArrayBoard;
use crate::shared::board::board_iterate::ro::BoardIterate;
use crate::shared::board::board_size::BoardSize;
use crate::shared::coord::directions::DOWN;
use crate::shared::coord::directions::RIGHT;
use crate::shared::coord::directions::UP;
use crate::shared::coord::point::Point;

use super::tictactoe::Symbol;

pub type Board = ArrayBoard<3, 3, Symbol>;

impl Board {
    pub fn corners<'a>(&'a self) -> impl Iterator<Item = Point> + 'a {
        let bottom = (0..self.width()).map(|i| Point::new(i, 0));
        let top = (0..self.width()).map(|i| Point::new(i, self.height() - 1));
        let left = (1..self.height() - 1).map(|j| Point::new(0, j));
        let right = (1..self.height() - 1).map(|j| Point::new(self.width() - 1, j));
        return bottom.chain(top).chain(left).chain(right);
    }

    pub fn lines<'a>(
        &'a self,
        start: Point,
    ) -> impl Iterator<Item = impl Iterator<Item = Point> + 'a> {
        [UP, RIGHT, UP + RIGHT, DOWN + RIGHT]
            .map(|direction| self.iterate(start, direction).map(|(point, _value)| point))
            .into_iter()
    }

    pub fn all_lines<'a>(&'a self) -> impl Iterator<Item = impl Iterator<Item = Point> + 'a> {
        self.corners().map(|corner| self.lines(corner)).flatten()
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::board::board_generate::BoardGenerate;
    use crate::tictactoe::tictactoe::Symbol;

    use super::Board;

    #[test]
    fn all_lines() {
        assert_eq!(
            8,
            Board::generate(|_| Symbol::None)
                .all_lines()
                .map(|line| line.count() == 3)
                .filter(|t| *t)
                .count()
        );
    }
}
