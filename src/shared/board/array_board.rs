use crate::shared::coord::point::Point;

use super::board_get_set::BoardGet;
use super::board_get_set::BoardSet;
use super::board_size::BoardSize;
use super::Board;

pub type ArrayBoard<const WIDTH: usize, const HEIGHT: usize, Value> =
    Board<[[Value; HEIGHT]; WIDTH], Value>;

impl<const WIDTH: usize, const HEIGHT: usize, Value> ArrayBoard<WIDTH, HEIGHT, Value> {
    pub fn generate(f: impl Fn(&Point) -> Value) -> Self {
        let array: [[Value; HEIGHT]; WIDTH] =
            std::array::from_fn(|i| std::array::from_fn(|j| f(&Point::new(i as i32, j as i32))));
        let result: Self = Self::new(array);
        result
    }
}

impl<const WIDTH: usize, const HEIGHT: usize, Value> BoardSize
    for ArrayBoard<WIDTH, HEIGHT, Value>
{
    fn width(&self) -> i32 {
        WIDTH as i32
    }

    fn height(&self) -> i32 {
        HEIGHT as i32
    }
}

impl<const WIDTH: usize, const HEIGHT: usize, Value> BoardGet for ArrayBoard<WIDTH, HEIGHT, Value> {
    type Value = Value;
    fn get(&self, point: &Point) -> &Value {
        &self.board[point.x as usize][point.y as usize]
    }
}

impl<const WIDTH: usize, const HEIGHT: usize, Value> BoardSet for ArrayBoard<WIDTH, HEIGHT, Value> {
    type Value = Value;
    fn get_mut(&mut self, point: &Point) -> &mut Value {
        &mut self.board[point.x as usize][point.y as usize]
    }
}
