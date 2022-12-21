use std::fmt::Debug;

use crate::shared::coord::point::Point;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ColoredPoint {
    color: char,
    point: Point,
}

impl Debug for ColoredPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ColoredPoint")
            .field("x", &self.x())
            .field("y", &self.y())
            .field("color", &self.color)
            .finish()
    }
}

impl ColoredPoint {
    pub fn new(x: i32, y: i32, color: char) -> Self {
        Self {
            point: Point::new(x, y),
            color,
        }
    }
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

    pub fn color_mut(&mut self) -> &mut char {
        &mut self.color
    }
    pub fn x_mut(&mut self) -> &mut i32 {
        &mut self.point.x
    }
    pub fn y_mut(&mut self) -> &mut i32 {
        &mut self.point.y
    }
}
