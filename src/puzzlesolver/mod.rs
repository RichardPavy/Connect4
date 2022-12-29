use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::Drop;

use crate::shared::board::array_board::ArrayBoard;
use crate::shared::board::board_get_set::BoardGet;
use crate::shared::board::board_get_set::BoardSet;
use crate::shared::board::board_size::BoardSize;
use crate::shared::coord::point::Point;

use self::shape::Shape;
use self::solution::Solution;
use self::solver_progress::SolverProgress;

mod shape;
mod solution;
mod solver_progress;
mod tagged_point;

pub type Board = ArrayBoard<8, 8, char>;

pub fn solve_puzzle() {
    let sprites = [
        "
            O
            XO
             X
        ",
        "
            O
            X
            OXO
        ",
        "
            X
            OX
             OX
        ",
        "
             O
            OXOX
        ",
        "
            O
            XOX
              O
        ",
        "
            XOX
             XO
        ",
        "
            XOX
              OX
        ",
        "
            X
        ",
        "
            X
            OXO
            X
        ",
        "
            XO
            OX
        ",
        "
            OXOX
               O
        ",
        "
             O
            OXO
             O
        ",
        "
            X X
            OXO
        ",
        "
            XOXOX
        ",
    ];
    let mut board = Board::generate(|point| {
        if (point.x + point.y) % 2 == 0 {
            'X'
        } else {
            'O'
        }
    });
    let solution = board.solve_puzzle(&sprites);
    println!("{}", solution);
}

impl Board {
    pub fn solve_puzzle(&mut self, sprites: &[&str]) -> Solution {
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
        return Solution::Solution(self.solve_puzzle_rec(&shapes, &mut progress).unwrap());
    }

    fn solve_puzzle_rec(
        &mut self,
        shapes: &[Vec<Shape>],
        progress: &mut SolverProgress,
    ) -> Option<Vec<(Shape, Point)>> {
        if shapes.is_empty() {
            return Some(vec![]);
        }
        let size = self.size();
        let shape = &shapes[0];
        for variant in shape {
            for i in 0..=(size.width() - variant.width()) {
                for j in 0..=(size.height() - variant.height()) {
                    let at = Point::new(i, j);
                    progress.incr();
                    if let Some(mut self2) = self.matches(variant, &at) {
                        if let Some(mut solution) = self2.solve_puzzle_rec(&shapes[1..], progress) {
                            solution.push((variant.clone(), at));
                            return Some(solution);
                        }
                    };
                }
            }
        }
        return None;
    }

    fn matches<'t>(&'t mut self, shape: &'t Shape, at: &'t Point) -> Option<TryVariant> {
        for tagged_point in &shape.tagged_points {
            if *self.get(&(*tagged_point.as_point() + *at)) != tagged_point.color() {
                return None;
            }
        }
        for tagged_point in &shape.tagged_points {
            *self.get_mut(&(*tagged_point.as_point() + *at)) = ' ';
        }
        Some(TryVariant {
            board: self,
            shape,
            at,
        })
    }
}

struct TryVariant<'t> {
    board: &'t mut Board,
    shape: &'t Shape,
    at: &'t Point,
}

impl<'t> Deref for TryVariant<'t> {
    type Target = Board;
    fn deref(&self) -> &Self::Target {
        &self.board
    }
}

impl<'t> DerefMut for TryVariant<'t> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.board
    }
}

impl<'t> Drop for TryVariant<'t> {
    fn drop(&mut self) {
        for tagged_point in &self.shape.tagged_points {
            *self.board.get_mut(&(*tagged_point.as_point() + *self.at)) = tagged_point.color();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Board;

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
    //#[test]
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
        let mut board = Board::generate(|point| {
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
