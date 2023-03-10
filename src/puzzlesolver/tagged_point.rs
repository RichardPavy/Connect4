use std::fmt::Debug;

use crate::shared::coord::point::Point;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TaggedPoint {
    color: char,
    point: Point,
}

impl Debug for TaggedPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ColoredPoint")
            .field("x", &self.x())
            .field("y", &self.y())
            .field("color", &self.color)
            .finish()
    }
}

impl TaggedPoint {
    pub fn new(x: i32, y: i32, color: char) -> Self {
        Self {
            point: Point::new(x, y),
            color,
        }
    }

    #[cfg(test)]
    pub fn new_pound(x: i32, y: i32) -> Self {
        Self::new(x, y, '#')
    }

    pub fn color(&self) -> char {
        self.color
    }
    pub fn x(&self) -> i32 {
        self.point.x
    }
    pub fn y(&self) -> i32 {
        self.point.y
    }
    pub fn as_point(&self) -> &Point {
        &self.point
    }

    pub fn x_mut(&mut self) -> &mut i32 {
        &mut self.point.x
    }
    pub fn y_mut(&mut self) -> &mut i32 {
        &mut self.point.y
    }
}
