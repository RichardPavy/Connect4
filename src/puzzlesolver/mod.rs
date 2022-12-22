use crate::shared::board::array_board::ArrayBoard;
use crate::shared::board::board_get_set::BoardGet;
use crate::shared::board::board_size::BoardSize;
use crate::shared::coord::point::Point;

use self::shape::Shape;
use self::solution::Solution;

mod shape;
mod solution;
mod tagged_point;

pub type Board = ArrayBoard<4, 4, char>;

impl Board {
    pub fn solve_puzzle(&mut self, sprites: &[&str]) -> Solution {
        let shapes: Vec<Vec<Shape>> = sprites
            .iter()
            .map(|sprite| Shape::parse(sprite).variants())
            .collect();
        return Solution::Solution(self.solve_puzzle_rec(&shapes).unwrap());
    }

    fn solve_puzzle_rec(&mut self, shapes: &[Vec<Shape>]) -> Option<Vec<(Shape, Point)>> {
        if shapes.is_empty() {
            return Some(vec![]);
        }
        let size = self.size();
        let shape = &shapes[0];
        for variant in shape {
            for i in 0..=(size.width() - variant.width()) {
                for j in 0..=(size.height() - variant.height()) {
                    let at = Point::new(i, j);
                    if self.matches(variant, &at) {
                        if let Some(mut solution) = self.solve_puzzle_rec(&shapes[1..]) {
                            solution.push((variant.clone(), at));
                            return Some(solution);
                        }
                    }
                }
            }
        }
        return None;
    }

    fn matches(&self, shape: &Shape, at: &Point) -> bool {
        for tagged_point in &shape.tagged_points {
            if *self.get(&(*tagged_point.as_point() + *at)) != tagged_point.color() {
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use crate::puzzlesolver::shape::Shape;
    use crate::shared::coord::point::Point;

    use super::solution;
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
        let mut board = Board::generate(|point| {
            if (point.x + point.y) % 2 == 0 {
                'X'
            } else {
                'O'
            }
        });
        let solution = board.solve_puzzle(&sprites);
        println!("{}", solution);
        assert_eq!("", format!("{}", solution));
    }
}
