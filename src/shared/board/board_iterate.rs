pub mod ro {
    use std::marker::PhantomData;

    use crate::shared::board::board_get_set::BoardGet;
    use crate::shared::board::board_size::BoardSize;
    use crate::shared::coord::point::Point;

    pub trait BoardIterate<Value> {
        fn iterate<'board>(
            &'board self,
            from: Point,
            direction: Point,
        ) -> LineIterator<'board, Self, Value> {
            LineIterator {
                board: self,
                point: from,
                direction,
                _phantom: PhantomData,
            }
        }
    }

    impl<TBoard: BoardSize, Value> BoardIterate<Value> for TBoard {}

    pub struct LineIterator<'board, B: BoardIterate<V> + ?Sized, V> {
        board: &'board B,
        point: Point,
        direction: Point,
        _phantom: PhantomData<V>,
    }

    impl<'board, B, V: 'board> Iterator for LineIterator<'board, B, V>
    where
        B: BoardIterate<V> + BoardGet<Value = V> + BoardSize,
    {
        type Item = (Point, &'board V);

        fn next(&mut self) -> Option<Self::Item> {
            if self.board.is_valid(&self.point) {
                let result = (self.point, self.board.get(&self.point));
                self.point = self.point + self.direction;
                Some(result)
            } else {
                None
            }
        }
    }
}

pub mod rw {
    use crate::shared::board::board_get_set::BoardSet;
    use crate::shared::board::board_size::BoardSize;
    use crate::shared::coord::point::Point;

    pub trait BoardIterateMut {
        fn iterate_mut<'board>(
            &'board mut self,
            from: Point,
            direction: Point,
        ) -> LineIteratorMut<'board, Self> {
            LineIteratorMut {
                board: self,
                point: from,
                direction,
            }
        }
    }

    impl<TBoard: BoardSize + BoardSet> BoardIterateMut for TBoard {}

    pub struct LineIteratorMut<'board, B: BoardIterateMut + ?Sized> {
        board: &'board mut B,
        point: Point,
        direction: Point,
    }

    impl<'board, B: 'board> Iterator for LineIteratorMut<'board, B>
    where
        B: BoardIterateMut + BoardSet + BoardSize,
    {
        type Item = (Point, &'board mut <B as BoardSet>::Value);

        fn next(&mut self) -> Option<Self::Item> {
            if self.board.is_valid(&self.point) {
                let curr_point = self.point;
                self.point = self.point + self.direction;
                let board = self.board as *mut B;
                let board = unsafe { &mut *board };
                let curr_value = board.get_mut(&curr_point);
                return Some((curr_point, curr_value));
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::shared::board::array_board::ArrayBoard;
    use crate::shared::board::board_get_set::BoardGet;
    use crate::shared::board::board_iterate::ro::BoardIterate;
    use crate::shared::board::board_iterate::rw::BoardIterateMut;
    use crate::shared::coord::point::Point;

    #[test]
    fn iterate() {
        type TestBoard = ArrayBoard<3, 5, i32>;
        let board: TestBoard = Default::default();
        assert_eq!(
            (vec![0, 1, 2], vec![0, 0, 0]),
            board
                .iterate(Point::new(0, 0), Point::new(1, 0))
                .map(|(point, _)| (point.x, point.y))
                .unzip()
        );
        assert_eq!(
            (vec![2, 1, 0], vec![0, 0, 0]),
            board
                .iterate(Point::new(2, 0), Point::new(-1, 0))
                .map(|(point, _)| (point.x, point.y))
                .unzip()
        );
        assert_eq!(
            (vec![0, 0, 0, 0, 0], vec![0, 1, 2, 3, 4]),
            board
                .iterate(Point::new(0, 0), Point::new(0, 1))
                .map(|(point, _)| (point.x, point.y))
                .unzip()
        );
        assert_eq!(
            (vec![0, 0, 0, 0, 0], vec![4, 3, 2, 1, 0]),
            board
                .iterate(Point::new(0, 4), Point::new(0, -1))
                .map(|(point, _)| (point.x, point.y))
                .unzip()
        );
        assert_eq!(
            (vec![1, 2], vec![3, 2]),
            board
                .iterate(Point::new(1, 3), Point::new(1, -1))
                .map(|(point, _)| (point.x, point.y))
                .unzip()
        );
    }

    #[test]
    fn iterate_mut() {
        type TestBoard = ArrayBoard<3, 5, i32>;
        let mut board: TestBoard = Default::default();
        board
            .iterate_mut(Point::new(0, 0), Point::new(1, 0))
            .for_each(|(_point, v)| *v = 1);
        assert_eq!(
            vec![1, 1, 1],
            board
                .iterate_mut(Point::new(0, 0), Point::new(1, 0))
                .map(|(_, v)| *v)
                .collect::<Vec<i32>>()
        );
        assert_eq!(
            vec![0, 0, 0],
            board
                .iterate_mut(Point::new(0, 1), Point::new(1, 0))
                .map(|(_, v)| *v)
                .collect::<Vec<i32>>()
        );

        let ref_value = board
            .iterate_mut(Point::new(0, 1), Point::new(1, 0))
            .next()
            .unwrap()
            .1;
        *ref_value = 345;
        assert_eq!(345, *board.get(&Point::new(0, 1)));
        // Above line works because the non-lexical lifetime scope of 'ref_value' ends before the immutable borrow of 'board' to call 'get'.
        // Using 'ref_value' again triggers error:
        // > "cannot borrow `board` as immutable because it is also borrowed as mutable"
        // *ref_value = 346
    }
}
