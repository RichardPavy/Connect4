use crate::shared::coord::point::Point;

use super::board_get_set::BoardSet;
use super::board_size::BoardSize;

pub trait BoardForAllPoints: BoardSize {
    fn for_all_points(&self, mut f: impl FnMut(Point)) {
        for i in 0..self.width() {
            for j in 0..self.height() {
                f(Point::new(i, j))
            }
        }
    }
}

impl<TBoard: BoardSize> BoardForAllPoints for TBoard {}

pub trait BoardForAllPointsMut: BoardSize + BoardSet {
    fn for_all_points_mut(&mut self, mut f: impl FnMut(Point, &mut Self::Value)) {
        for i in 0..self.width() {
            for j in 0..self.height() {
                let point = Point::new(i, j);
                f(point, self.get_mut(&point))
            }
        }
    }
}

impl<TBoard: BoardSize + BoardSet> BoardForAllPointsMut for TBoard {}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::shared::board::array_board::ArrayBoard;
    use crate::shared::board::board_for_all_points::BoardForAllPoints;
    use crate::shared::board::board_for_all_points::BoardForAllPointsMut;
    use crate::shared::board::board_get_set::BoardGet;
    use crate::shared::coord::point::Point;

    #[test]
    fn for_all_points() {
        type TestBoard = ArrayBoard<2, 3, i32>;
        let board: TestBoard = Default::default();
        let mut x = vec![];
        let mut y = vec![];
        board.for_all_points(|point| x.push(point.x));
        board.for_all_points(|point| y.push(point.y));
        assert_eq!(vec![0, 0, 0, 1, 1, 1], x);
        assert_eq!(vec![0, 1, 2, 0, 1, 2], y);
    }

    #[test]
    fn for_all_points_mut() {
        type TestBoard = ArrayBoard<2, 3, i32>;
        let mut board: TestBoard = Default::default();
        board.for_all_points_mut(|p, v| *v = p.x + p.y);
        assert_eq!(0, *board.get(&Point::new(0, 0)));
        assert_eq!(1, *board.get(&Point::new(0, 1)));
        assert_eq!(1, *board.get(&Point::new(1, 0)));
        assert_eq!(3, *board.get(&Point::new(1, 2)));
    }
}
