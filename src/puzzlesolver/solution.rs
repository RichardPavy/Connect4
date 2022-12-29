use std::fmt::Display;
use std::marker::PhantomData;

use crate::shared::board::board_generate::BoardGenerate;
use crate::shared::coord::point::Point;

use super::shape::Shape;
use super::solver::Puzzle;

pub struct Solution<TPuzzle: Puzzle> {
    positioned_shapes: Vec<(Shape, Point)>,
    _phantom: PhantomData<TPuzzle>,
}

impl<TPuzzle: Puzzle> Solution<TPuzzle> {
    pub fn of(positioned_shapes: Vec<(Shape, Point)>) -> Self {
        Self {
            positioned_shapes,
            _phantom: PhantomData,
        }
    }
}

impl<TPuzzle: Puzzle + BoardGenerate<Value = char>> Display for Solution<TPuzzle> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            positioned_shapes, ..
        } = self;
        for (i, (shape, at)) in positioned_shapes.iter().enumerate() {
            if i != 0 {
                writeln!(f)?;
            };
            let board = TPuzzle::generate(|point| {
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
        }
        Ok(())
    }
}
