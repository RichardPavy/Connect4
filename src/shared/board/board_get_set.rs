use crate::shared::coord::point::Point;

pub trait BoardGet {
    type Value;
    fn get(&self, point: &Point) -> &Self::Value;
}

pub trait BoardSet {
    type Value;
    fn get_mut(&mut self, point: &Point) -> &mut Self::Value;
    fn set(&mut self, point: &Point, value: Self::Value) -> &mut Self {
        *self.get_mut(point) = value;
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::board::array_board::ArrayBoard;
    use crate::shared::board::board_get_set::BoardGet;
    use crate::shared::board::board_get_set::BoardSet;
    use crate::shared::coord::point::Point;

    #[test]
    fn get_set() {
        type TestBoard = ArrayBoard<3, 5, i32>;
        let mut board: TestBoard = Default::default();
        assert_eq!(&0, board.get(&Point::new(0, 0)));
        assert_eq!(
            &123,
            board.set(&Point::new(0, 0), 123).get(&Point::new(0, 0))
        );
        assert_eq!(
            &456,
            board.set(&Point::new(2, 4), 456).get(&Point::new(2, 4))
        );
    }
}
