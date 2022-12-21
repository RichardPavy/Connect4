use crate::shared::coord::point::Point;

pub trait BoardSize {
    fn width(&self) -> i32;
    fn height(&self) -> i32;
    fn is_valid(&self, point: &Point) -> bool {
        point.x >= 0 && point.x < self.width() && point.y >= 0 && point.y < self.height()
    }
    fn size(&self) -> Size {
        Size::new(self.width(), self.height())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Size {
    point: Point,
}

impl Size {
    pub fn new(x: i32, y: i32) -> Size {
        Size {
            point: Point { x, y },
        }
    }
}

impl BoardSize for Size {
    fn width(&self) -> i32 {
        let Size {
            point: Point { x, .. },
        } = self;
        *x
    }

    fn height(&self) -> i32 {
        let Size {
            point: Point { y, .. },
        } = self;
        *y
    }
}
