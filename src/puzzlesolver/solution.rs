use std::fmt::Display;

use crate::shared::coord::point::Point;

use super::shape::Shape;
use super::Board;

pub enum Solution {
    Solution(Vec<(Shape, Point)>),
}

impl Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Solution::Solution(shapes) = self;
        for (shape, at) in shapes {
            let board = Board::generate(|point| {
                if let Some(color) = shape
                    .tagged_points
                    .iter()
                    .filter(|tagged_point| *tagged_point.as_point() + *at == *point)
                    .map(|tagged_point| tagged_point.color())
                    .next()
                {
                    color
                } else {
                    ' '
                }
            });
            write!(f, "{}", board)?;
            writeln!(f)?;
        }
        Ok(())
    }
}
