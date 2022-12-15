use crate::shared::coord::point::Point;

use super::board_size::BoardSize;
use super::board_size::Size;

pub trait BoardCorners: BoardSize {
    fn corners(&self) -> BoardCornersIterator {
        BoardCornersIterator {
            size: self.size(),
            idx: -1,
            side: Side::Down,
        }
    }
}

impl<TBoard: BoardSize> BoardCorners for TBoard {}

pub struct BoardCornersIterator {
    size: Size,
    idx: i32,
    side: Side,
}

impl Iterator for BoardCornersIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let next_idx = self.idx + 1;
        match get_next(&self.size, next_idx, self.side) {
            Next::Point(point) => {
                self.idx = next_idx;
                Some(point)
            }
            Next::Side {
                next_side: Side::Down,
                ..
            } => None,
            Next::Side {
                next_from,
                next_side,
            } => {
                self.idx = next_from - 1;
                self.side = next_side;
                self.next()
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Side {
    Down,
    Up,
    Left,
    Right,
}

enum Next {
    Point(Point),
    Side { next_from: i32, next_side: Side },
}

fn get_next(size: &Size, idx: i32, side: Side) -> Next {
    let (to, next_point, next_from, next_side) = match side {
        Side::Down => (size.width(), Point::new(idx, 0), 0, Side::Up),
        Side::Up => (
            size.width(),
            Point::new(idx, size.height() - 1),
            1,
            Side::Left,
        ),
        Side::Left => (size.height() - 1, Point::new(0, idx), 1, Side::Right),
        Side::Right => (
            size.height() - 1,
            Point::new(size.width() - 1, idx),
            i32::MAX,
            Side::Down,
        ),
    };
    if idx == to {
        Next::Side {
            next_from,
            next_side,
        }
    } else {
        Next::Point(next_point)
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::shared::board::array_board::ArrayBoard;
    use crate::shared::board::board_corners::BoardCorners;
    use crate::shared::board::board_get_set::BoardSet;
    use crate::shared::coord::point::Point;

    #[test]
    fn all_points33() {
        type TestBoard = ArrayBoard<3, 3, char>;
        let mut board: TestBoard = TestBoard::generate(|_| ' ');
        let x: Vec<i32> = board.corners().map(|Point { x, y: _ }| x).collect();
        let y: Vec<i32> = board.corners().map(|Point { x: _, y }| y).collect();
        assert_eq!(vec![0, 1, 2, 0, 1, 2, 0, 2], x);
        assert_eq!(vec![0, 0, 0, 2, 2, 2, 1, 1], y);

        for point in board.corners() {
            board.set(&point, 'X');
        }
        println!("{}", board);
        assert_eq!(
            r#"
    | 1 | 2 | 3 
----+---+---+---
 A  | X | X | X 
----+---+---+---
 B  | X |   | X 
----+---+---+---
 C  | X | X | X 
"#,
            format!("\n{board}")
        );
    }

    #[test]
    fn all_points34() {
        type TestBoard = ArrayBoard<4, 3, char>;
        let mut board: TestBoard = TestBoard::generate(|_| ' ');
        let x: Vec<i32> = board.corners().map(|Point { x, y: _ }| x).collect();
        let y: Vec<i32> = board.corners().map(|Point { x: _, y }| y).collect();
        assert_eq!(vec![0, 1, 2, 3, 0, 1, 2, 3, 0, 3], x);
        assert_eq!(vec![0, 0, 0, 0, 2, 2, 2, 2, 1, 1], y);

        for point in board.corners() {
            board.set(&point, 'X');
        }
        println!("{}", board);
        assert_eq!(
            r#"
    | 1 | 2 | 3 | 4 
----+---+---+---+---
 A  | X | X | X | X 
----+---+---+---+---
 B  | X |   |   | X 
----+---+---+---+---
 C  | X | X | X | X 
"#,
            format!("\n{board}")
        );
    }
}
