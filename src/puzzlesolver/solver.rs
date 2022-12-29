use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::Drop;

use crate::shared::board::board_get_set::BoardGet;
use crate::shared::board::board_get_set::BoardSet;
use crate::shared::board::board_size::BoardSize;
use crate::shared::coord::point::Point;

use super::shape::Shape;
use super::solution::Solution;
use super::solver_progress::SolverProgress;

impl<TBoard: BoardGet<Value = char> + BoardSet<Value = char> + BoardSize + std::fmt::Display> Puzzle
    for TBoard
{
}

pub trait Puzzle:
    BoardGet<Value = char> + BoardSet<Value = char> + BoardSize + std::fmt::Display + Sized
{
    fn solve_puzzle(&mut self, sprites: &[&str]) -> Solution<Self> {
        let shapes: Vec<Vec<Shape>> = sprites
            .iter()
            .map(|sprite| Shape::parse(sprite).variants())
            .collect();
        {
            let shapes_count = shapes.len();
            let variants_count: usize = shapes.iter().map(|v| v.len()).sum();
            println!("Got {shapes_count} shapes with {variants_count} variants");
        }
        let mut progress = SolverProgress::new();
        return Solution::of(solve_puzzle_rec(self, &shapes, &mut progress).unwrap());
    }
}

fn solve_puzzle_rec(
    puzzle: &mut impl Puzzle,
    shapes: &[Vec<Shape>],
    progress: &mut SolverProgress,
) -> Option<Vec<(Shape, Point)>> {
    if shapes.is_empty() {
        return Some(vec![]);
    }
    let size = puzzle.size();
    let shape = &shapes[0];
    for variant in shape {
        for i in 0..=(size.width() - variant.width()) {
            for j in 0..=(size.height() - variant.height()) {
                let at = Point::new(i, j);
                progress.incr();
                if let Some(mut puzzle) = matches(puzzle, variant, &at) {
                    if let Some(mut solution) =
                        solve_puzzle_rec(puzzle.deref_mut(), &shapes[1..], progress)
                    {
                        solution.push((variant.clone(), at));
                        return Some(solution);
                    }
                };
            }
        }
    }
    return None;
}

fn matches<'t, TPuzzle: Puzzle>(
    puzzle: &'t mut TPuzzle,
    shape: &'t Shape,
    at: &'t Point,
) -> Option<TryVariant<'t, TPuzzle>> {
    for tagged_point in &shape.tagged_points {
        if *puzzle.get(&(*tagged_point.as_point() + *at)) != tagged_point.color() {
            return None;
        }
    }
    for tagged_point in &shape.tagged_points {
        *puzzle.get_mut(&(*tagged_point.as_point() + *at)) = ' ';
    }
    Some(TryVariant { puzzle, shape, at })
}

struct TryVariant<'t, TPuzzle: Puzzle> {
    puzzle: &'t mut TPuzzle,
    shape: &'t Shape,
    at: &'t Point,
}

impl<'t, TPuzzle: Puzzle> Deref for TryVariant<'t, TPuzzle> {
    type Target = TPuzzle;
    fn deref(&self) -> &Self::Target {
        &self.puzzle
    }
}

impl<'t, TPuzzle: Puzzle> DerefMut for TryVariant<'t, TPuzzle> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.puzzle
    }
}

impl<'t, TPuzzle: Puzzle> Drop for TryVariant<'t, TPuzzle> {
    fn drop(&mut self) {
        for tagged_point in &self.shape.tagged_points {
            *self.puzzle.get_mut(&(*tagged_point.as_point() + *self.at)) = tagged_point.color();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzlesolver::solver::Puzzle;
    use crate::shared::board::array_board::ArrayBoard;
    use crate::shared::board::board_generate::BoardGenerate;

    // The end board should look like:
    //  X | X | U | U
    // ---+---+---+---
    //  O | X | U | Z
    // ---+---+---+---
    //  O | X | M | M
    // ---+---+---+---
    //  O | O | M | M
    //
    // The sprites are:
    // XO   XO   X   O    XO
    //  X   O        X    OX
    //  O            OX
    #[test]
    fn solve_puzzle() {
        let sprites = [
            "
                XO
                 X
                 O
            ",
            "
                XO
                O
            ",
            "X",
            "
                O
                X
                OX
            ",
            "
                OX
                XO
            ",
        ];
        let mut board = ArrayBoard::<4, 4, char>::generate(|point| {
            if (point.x + point.y) % 2 == 0 {
                'X'
            } else {
                'O'
            }
        });
        let solution = board.solve_puzzle(&sprites);
        println!("{}", solution);
        assert_eq!(
            "
    | 1 | 2 | 3 | 4 
----+---+---+---+---
 A  |   |   |   |   
----+---+---+---+---
 B  |   |   |   |   
----+---+---+---+---
 C  |   |   | X | O 
----+---+---+---+---
 D  |   |   | O | X 

    | 1 | 2 | 3 | 4 
----+---+---+---+---
 A  |   | O | X | O 
----+---+---+---+---
 B  |   |   |   | X 
----+---+---+---+---
 C  |   |   |   |   
----+---+---+---+---
 D  |   |   |   |   

    | 1 | 2 | 3 | 4 
----+---+---+---+---
 A  |   |   |   |   
----+---+---+---+---
 B  |   |   |   |   
----+---+---+---+---
 C  |   |   |   |   
----+---+---+---+---
 D  |   | X |   |   

    | 1 | 2 | 3 | 4 
----+---+---+---+---
 A  |   |   |   |   
----+---+---+---+---
 B  |   |   |   |   
----+---+---+---+---
 C  | X | O |   |   
----+---+---+---+---
 D  | O |   |   |   

    | 1 | 2 | 3 | 4 
----+---+---+---+---
 A  | X |   |   |   
----+---+---+---+---
 B  | O | X | O |   
----+---+---+---+---
 C  |   |   |   |   
----+---+---+---+---
 D  |   |   |   |   
",
            format!("\n{}", solution)
        );
    }
}