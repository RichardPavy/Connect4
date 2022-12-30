use std::fmt::Display;
use std::marker::PhantomData;

use thousands::Separable;

use crate::shared::board::board_generate::BoardGenerate;
use crate::shared::coord::point::Point;

use super::puzzle_piece::PuzzlePiece;
use super::shape::Shape;
use super::solver::Puzzle;

pub(super) struct Solution<TPuzzle: Puzzle> {
    positioned_shapes: Vec<(Shape, Point)>,
    iterations: u64,
    _phantom: PhantomData<TPuzzle>,
}

impl<TPuzzle: Puzzle> Solution<TPuzzle> {
    pub fn of(mut positioned_shapes: Vec<(usize, Shape, Point)>, iterations: u64) -> Self {
        positioned_shapes.sort_by_key(|(shape_idx, _shape, _at)| *shape_idx);
        Self {
            positioned_shapes: positioned_shapes
                .into_iter()
                .map(|(_shape_idx, shape, at)| (shape, at))
                .collect(),
            iterations,
            _phantom: PhantomData,
        }
    }
}

impl<TPuzzle: Puzzle + BoardGenerate<Value = PuzzlePiece>> Display for Solution<TPuzzle> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Found solution after {} iterations.",
            self.iterations.separate_with_spaces()
        )?;
        writeln!(f)?;
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
                    PuzzlePiece::blank_char(color)
                } else {
                    PuzzlePiece::blank_char(' ')
                }
            });
            write!(f, "{}", board)?;
        }
        Ok(())
    }
}
