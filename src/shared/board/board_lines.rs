use crate::shared::coord::point::Point;

use super::board_size::BoardSize;
use super::board_size::Size;

pub trait BoardLines: BoardSize {
    fn lines(&self, start: Point, direction: Point, length: i32) -> LinesIterator {
        LinesIterator {
            size: self.size(),
            start: start - direction * (length - 1),
            direction,
            length,
            idx: 0,
        }
    }
}

impl<TBoard: BoardSize> BoardLines for TBoard {}

pub struct LinesIterator {
    size: Size,
    start: Point,
    direction: Point,
    length: i32,
    idx: i32,
}

impl Iterator for LinesIterator {
    type Item = LineIterator;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.idx == self.length {
                return None;
            }

            let pos_from = self.start + self.direction * self.idx;
            let pos_to = pos_from + self.direction * (self.length - 1);
            self.idx += 1;

            if self.size.is_valid(&pos_from) && self.size.is_valid(&pos_to) {
                return Some(LineIterator {
                    direction: self.direction,
                    point: pos_from,
                    count: self.length,
                });
            }
        }
    }
}

pub struct LineIterator {
    direction: Point,
    point: Point,
    count: i32,
}

impl Iterator for LineIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 0 {
            None
        } else {
            let curr_point = self.point;
            self.point = self.point + self.direction;
            self.count -= 1;
            Some(curr_point)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::shared::board::array_board::ArrayBoard;
    use crate::shared::board::board_lines::BoardLines;
    use crate::shared::coord::directions;
    use crate::shared::coord::point::Point;

    #[test]
    fn lines() {
        let board: ArrayBoard<4, 3, i32> = Default::default();
        assert_eq!(
            vec![vec![(0, 0), (0, 1)]],
            get_lines(board.lines(Point::new(0, 0), directions::UP, 2))
        );
        assert_eq!(
            vec![vec![(0, 1), (0, 0)]],
            get_lines(board.lines(Point::new(0, 0), directions::DOWN, 2))
        );
        assert_eq!(
            vec![vec![(0, 0), (0, 1)], vec![(0, 1), (0, 2)]],
            get_lines(board.lines(Point::new(0, 1), directions::UP, 2))
        );
        assert_eq!(
            vec![vec![(0, 2), (0, 1)], vec![(0, 1), (0, 0)]],
            get_lines(board.lines(Point::new(0, 1), directions::DOWN, 2))
        );
    }

    #[test]
    fn large() {
        let board: ArrayBoard<20, 20, i32> = Default::default();
        assert_eq!(
            vec![
                vec![(0, 7), (0, 8), (0, 9), (0, 10)],
                vec![(0, 8), (0, 9), (0, 10), (0, 11)],
                vec![(0, 9), (0, 10), (0, 11), (0, 12)],
                vec![(0, 10), (0, 11), (0, 12), (0, 13)],
            ],
            get_lines(board.lines(Point::new(0, 10), directions::UP, 4))
        );
        assert_eq!(
            vec![
                vec![(0, 13), (0, 12), (0, 11), (0, 10)],
                vec![(0, 12), (0, 11), (0, 10), (0, 9)],
                vec![(0, 11), (0, 10), (0, 9), (0, 8)],
                vec![(0, 10), (0, 9), (0, 8), (0, 7)],
            ],
            get_lines(board.lines(Point::new(0, 10), directions::DOWN, 4))
        );
    }

    #[test]
    fn diagonal() {
        let board: ArrayBoard<20, 20, i32> = Default::default();
        assert_eq!(
            vec![
                vec![(13, 7), (12, 8), (11, 9), (10, 10)],
                vec![(12, 8), (11, 9), (10, 10), (9, 11)],
                vec![(11, 9), (10, 10), (9, 11), (8, 12)],
                vec![(10, 10), (9, 11), (8, 12), (7, 13)],
            ],
            get_lines(board.lines(Point::new(10, 10), directions::UP + directions::LEFT, 4))
        );
    }

    #[test]
    fn edge() {
        let board: ArrayBoard<20, 20, i32> = Default::default();
        assert_eq!(
            vec![
                vec![(0, 0), (1, 1), (2, 2), (3, 3)],
                vec![(1, 1), (2, 2), (3, 3), (4, 4)],
                vec![(2, 2), (3, 3), (4, 4), (5, 5)],
            ],
            get_lines(board.lines(Point::new(2, 2), directions::UP + directions::RIGHT, 4))
        );
    }

    fn get_lines(lines: impl Iterator<Item = impl Iterator<Item = Point>>) -> Vec<Vec<(i32, i32)>> {
        return lines
            .map(|line| line.map(|Point { x, y }| (x, y)).collect())
            .collect();
    }
}
