use std::fmt::Display;

use crate::shared::board::board_size::BoardSize;
use crate::shared::board::board_size::Size;
use crate::shared::coord::point::Point;

use super::tagged_point::TaggedPoint;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Shape {
    pub size: Size,
    pub tagged_points: Vec<TaggedPoint>,
}

impl Shape {
    pub fn new(points: Vec<TaggedPoint>) -> Self {
        let mut result = Self {
            size: Size::new(0, 0),
            tagged_points: points,
        };
        result.normalize();
        return result;
    }

    pub fn parse(sprite: &str) -> Self {
        let mut points = vec![];
        for (i, line) in sprite.lines().enumerate() {
            for (j, char) in line.char_indices() {
                if char != ' ' {
                    points.push(TaggedPoint::new(i as i32, j as i32, char));
                }
            }
        }
        return Self::new(points);
    }

    fn normalize(&mut self) {
        if self.tagged_points.is_empty() {
            self.size = Size::new(0, 0);
            return;
        }
        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;
        for point in &self.tagged_points {
            min_x = min_x.min(point.x());
            min_y = min_y.min(point.y());
            max_x = max_x.max(point.x());
            max_y = max_y.max(point.y());
        }
        self.translate(&Point::new(-min_x, -min_y));
        self.size = Size::new(max_x - min_x + 1, max_y - min_y + 1);
    }

    pub fn translate(&mut self, direction: &Point) {
        for point in &mut self.tagged_points {
            *point.x_mut() += direction.x;
            *point.y_mut() += direction.y;
        }
    }

    pub fn rotate_left(&mut self) {
        let width = self.width();
        for point in &mut self.tagged_points {
            (*point.x_mut(), *point.y_mut()) = (point.y(), width - point.x() - 1);
        }
        self.size = Size::new(self.height(), self.width());
    }

    #[cfg(test)]
    pub fn rotate_right(&mut self) {
        let height = self.height();
        for point in &mut self.tagged_points {
            (*point.x_mut(), *point.y_mut()) = (height - point.y() - 1, point.x());
        }
        self.size = Size::new(self.height(), self.width());
    }

    pub fn mirror_x(&mut self) {
        let width = self.width();
        for point in &mut self.tagged_points {
            *point.x_mut() = width - point.x() - 1;
        }
    }

    #[cfg(test)]
    pub fn mirror_y(&mut self) {
        let height = self.height();
        for point in &mut self.tagged_points {
            *point.y_mut() = height - point.y() - 1;
        }
    }

    pub fn variants(&self) -> Vec<Shape> {
        let mut shape = self.clone();
        let mut result = vec![];
        for _ in 0..2 {
            for _ in 0..4 {
                shape.tagged_points.sort();
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
        for j in 0..self.height() {
            for i in 0..self.width() {
                if let Some(point) = self
                    .tagged_points
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

    use crate::puzzlesolver::shape::TaggedPoint;
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
            shape.tagged_points,
            vec![
                TaggedPoint::new_pound(0, 0),
                TaggedPoint::new_pound(1, 0),
                TaggedPoint::new_pound(1, 1),
                TaggedPoint::new_pound(1, 2),
            ]
        );
    }

    #[test]
    fn new_negative_points() {
        let shape = Shape::new(vec![
            TaggedPoint::new_pound(2 - 10, 3 - 10),
            TaggedPoint::new_pound(3 - 10, 3 - 10),
            TaggedPoint::new_pound(3 - 10, 4 - 10),
            TaggedPoint::new_pound(3 - 10, 5 - 10),
        ]);
        assert_eq!(shape.size(), Size::new(2, 3));
        assert_eq!(
            shape.tagged_points,
            vec![
                TaggedPoint::new_pound(0, 0),
                TaggedPoint::new_pound(1, 0),
                TaggedPoint::new_pound(1, 1),
                TaggedPoint::new_pound(1, 2),
            ]
        );
    }

    #[test]
    fn to_string() {
        let shape = test_shape();
        println!("{}", shape);
        assert_eq!(
            "
##
 #
 #
",
            format!("\n{}", shape)
        );
    }

    #[test]
    fn rotate_left() {
        let mut shape = test_shape();
        shape.rotate_left();
        println!("{}", shape);
        assert_eq!(
            "
###
#  
",
            format!("\n{}", shape)
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
###
",
            format!("\n{}", shape)
        );
    }

    #[test]
    fn mirror_x() {
        let mut shape = test_shape();
        shape.mirror_x();
        println!("{}", shape);
        assert_eq!(
            "
##
# 
# 
",
            format!("\n{}", shape)
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
 #
##
",
            format!("\n{}", shape)
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
##
 #
 #

###
#  

# 
# 
##

  #
###

##
# 
# 

#  
###

 #
 #
##

###
  #
",
            format!("\n{}", variants)
        );
    }

    #[test]
    fn variants2() {
        let mut test_shape = test_shape();
        test_shape.tagged_points.push(TaggedPoint::new_pound(0, 2));
        let variants = test_shape
            .variants()
            .into_iter()
            .map(|shape| shape.to_string())
            .collect::<Vec<String>>()
            .join("\n");
        println!("{}", variants);
        assert_eq!(
            "
##
 #
##

###
# #

##
# 
##

# #
###
",
            format!("\n{}", variants)
        );
    }

    #[test]
    fn parse() {
        let sprite = "
        ####
        ###
        ##
        #";
        let actual = Shape::parse(sprite);
        let expected = {
            let mut points = vec![];
            for i in 0..4 {
                for j in 0..(4 - i) {
                    points.push(TaggedPoint::new(i, j, '#'));
                }
            }
            Shape::new(points)
        };
        assert_eq!(expected, actual);
    }

    fn test_shape() -> Shape {
        Shape::new(vec![
            TaggedPoint::new_pound(2, 3),
            TaggedPoint::new_pound(3, 3),
            TaggedPoint::new_pound(3, 4),
            TaggedPoint::new_pound(3, 5),
        ])
    }
}
