use std::fmt::Display;

use crate::shared::board::board_size::BoardSize;
use crate::shared::board::board_size::Size;
use crate::shared::coord::point::Point;

use super::colored_point::ColoredPoint;

#[derive(Clone, PartialEq, Eq)]
pub struct Shape {
    size: Size,
    points: Vec<ColoredPoint>,
}

impl Shape {
    pub fn new(points: Vec<ColoredPoint>) -> Self {
        let mut result = Self {
            size: Size::new(0, 0),
            points,
        };
        result.normalize();
        return result;
    }

    fn normalize(&mut self) {
        if self.points.is_empty() {
            self.size = Size::new(0, 0);
            return;
        }
        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;
        for point in &self.points {
            min_x = min_x.min(point.x());
            min_y = min_y.min(point.y());
            max_x = max_x.max(point.x());
            max_y = max_y.max(point.y());
        }
        self.translate(Point::new(-min_x, -min_y));
        self.size = Size::new(max_x - min_x + 1, max_y - min_y + 1);
    }

    fn translate(&mut self, direction: Point) {
        for point in &mut self.points {
            *point.x_mut() += direction.x;
            *point.y_mut() += direction.y;
        }
    }

    pub fn rotate_left(&mut self) {
        let width = self.width();
        for point in &mut self.points {
            (*point.x_mut(), *point.y_mut()) = (point.y(), width - point.x() - 1);
        }
        self.size = Size::new(self.height(), self.width());
    }

    pub fn rotate_right(&mut self) {
        let height = self.height();
        for point in &mut self.points {
            (*point.x_mut(), *point.y_mut()) = (height - point.y() - 1, point.x());
        }
        self.size = Size::new(self.height(), self.width());
    }

    pub fn mirror_x(&mut self) {
        let width = self.width();
        for point in &mut self.points {
            *point.x_mut() = width - point.x() - 1;
        }
    }

    pub fn mirror_y(&mut self) {
        let height = self.height();
        for point in &mut self.points {
            *point.y_mut() = height - point.y() - 1;
        }
    }

    pub fn variants(&self) -> Vec<Shape> {
        let mut shape = self.clone();
        let mut result = vec![];
        for _ in 0..2 {
            for _ in 0..4 {
                shape.points.sort();
                if !result.contains(&shape) {
                    result.push(shape.clone());
                }
                shape.rotate_left();
            }
            shape.mirror_x();
        }
        result
    }
}

impl BoardSize for Shape {
    fn width(&self) -> i32 {
        self.size.width()
    }

    fn height(&self) -> i32 {
        self.size.height()
    }
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = String::new();
        for i in 0..self.width() {
            for j in 0..self.height() {
                if let Some(point) = self
                    .points
                    .iter()
                    .find(|point| point.x() == i && point.y() == j)
                {
                    buf.push(point.color());
                } else {
                    buf.push(' ');
                }
            }
            buf.push('\n');
        }
        f.write_str(&buf)
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::puzzlesolver::shape::ColoredPoint;
    use crate::shared::board::board_size::BoardSize;
    use crate::shared::board::board_size::Size;

    use super::Shape;

    #[test]
    fn new_empty() {
        let shape = Shape::new(vec![]);
        assert_eq!(shape.size(), Size::new(0, 0));
    }

    #[test]
    fn new_non_empty() {
        let shape = test_shape();
        assert_eq!(shape.size(), Size::new(2, 3));
        assert_eq!(
            shape.points,
            vec![
                ColoredPoint::new_pound(0, 0),
                ColoredPoint::new_pound(1, 0),
                ColoredPoint::new_pound(1, 1),
                ColoredPoint::new_pound(1, 2),
            ]
        );
    }

    #[test]
    fn new_negative_points() {
        let shape = Shape::new(vec![
            ColoredPoint::new_pound(2 - 10, 3 - 10),
            ColoredPoint::new_pound(3 - 10, 3 - 10),
            ColoredPoint::new_pound(3 - 10, 4 - 10),
            ColoredPoint::new_pound(3 - 10, 5 - 10),
        ]);
        assert_eq!(shape.size(), Size::new(2, 3));
        assert_eq!(
            shape.points,
            vec![
                ColoredPoint::new_pound(0, 0),
                ColoredPoint::new_pound(1, 0),
                ColoredPoint::new_pound(1, 1),
                ColoredPoint::new_pound(1, 2),
            ]
        );
    }

    #[test]
    fn to_string() {
        let shape = test_shape();
        println!("{}", shape);
        assert_eq!(
            "
#  
###
",
            "\n".to_string() + &shape.to_string()
        );
    }

    #[test]
    fn rotate_left() {
        let mut shape = test_shape();
        shape.rotate_left();
        println!("{}", shape);
        assert_eq!(
            "
##
# 
# 
",
            "\n".to_string() + &shape.to_string()
        );
    }

    #[test]
    fn rotate_right() {
        let mut shape = test_shape();
        shape.rotate_right();
        println!("{}", shape);
        assert_eq!(
            "
 #
 #
##
",
            "\n".to_string() + &shape.to_string()
        );
    }

    #[test]
    fn mirror_x() {
        let mut shape = test_shape();
        shape.mirror_x();
        println!("{}", shape);
        assert_eq!(
            "
###
#  
",
            "\n".to_string() + &shape.to_string()
        );
    }

    #[test]
    fn mirror_y() {
        let mut shape = test_shape();
        shape.mirror_y();
        println!("{}", shape);
        assert_eq!(
            "
  #
###
",
            "\n".to_string() + &shape.to_string()
        );
    }

    #[test]
    fn variants() {
        let variants = test_shape()
            .variants()
            .into_iter()
            .map(|shape| shape.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        println!("{}", variants);
        assert_eq!(
            "
#  
###

##
# 
# 

###
  #

 #
 #
##

###
#  

##
 #
 #

  #
###

# 
# 
##
",
            "\n".to_owned() + &variants
        );
    }

    #[test]
    fn variants2() {
        let mut test_shape = test_shape();
        test_shape.points.push(ColoredPoint::new_pound(0, 2));
        let variants = test_shape
            .variants()
            .into_iter()
            .map(|shape| shape.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        println!("{}", variants);
        assert_eq!(
            "
# #
###

##
# 
##

###
# #

##
 #
##
",
            "\n".to_owned() + &variants
        );
    }

    fn test_shape() -> Shape {
        Shape::new(vec![
            ColoredPoint::new_pound(2, 3),
            ColoredPoint::new_pound(3, 3),
            ColoredPoint::new_pound(3, 4),
            ColoredPoint::new_pound(3, 5),
        ])
    }
}
