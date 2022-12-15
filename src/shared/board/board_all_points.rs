use crate::shared::coord::point::Point;

use super::board_size::BoardSize;
use super::board_size::Size;

pub trait BoardAllPoints: BoardSize {
    fn all_points(&self) -> AllPointsIterator {
        AllPointsIterator {
            size: self.size(),
            point: Point::new(-1, 0),
        }
    }
}

impl<TBoard: BoardSize> BoardAllPoints for TBoard {}

pub struct AllPointsIterator {
    size: Size,
    point: Point,
}

impl Iterator for AllPointsIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        self.point.x += 1;
        if self.point.x == self.size.width() {
            self.point.x = 0;
            if self.point.y + 1 == self.size.height() {
                return None;
            }
            self.point.y += 1;
        }
        Some(self.point)
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::shared::board::array_board::ArrayBoard;
    use crate::shared::board::board_all_points::BoardAllPoints;
    use crate::shared::board::board_get_set::BoardGet;
    use crate::shared::board::board_get_set::BoardSet;
    use crate::shared::coord::point::Point;

    #[test]
    fn all_points() {
        type TestBoard = ArrayBoard<2, 3, i32>;
        let board: TestBoard = Default::default();
        let x: Vec<i32> = board.all_points().map(|Point { x, y: _ }| x).collect();
        let y: Vec<i32> = board.all_points().map(|Point { x: _, y }| y).collect();
        assert_eq!(vec![0, 1, 0, 1, 0, 1], x);
        assert_eq!(vec![0, 0, 1, 1, 2, 2], y);
    }

    #[test]
    fn fall_points_mut() {
        type TestBoard = ArrayBoard<2, 3, i32>;
        let mut board: TestBoard = Default::default();
        board
            .all_points()
            .for_each(|p| *board.get_mut(&p) = p.x + p.y);
        assert_eq!(0, *board.get(&Point::new(0, 0)));
        assert_eq!(1, *board.get(&Point::new(0, 1)));
        assert_eq!(1, *board.get(&Point::new(1, 0)));
        assert_eq!(3, *board.get(&Point::new(1, 2)));
    }
}
