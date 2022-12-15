use std::fmt::Debug;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

pub mod directions;
pub mod point;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coord<T> {
    pub x: T,
    pub y: T,
}

impl<T> Coord<T> {
    pub const fn new(x: T, y: T) -> Coord<T> {
        Coord { x, y }
    }
}

macro_rules! coord_coord_op {
    ($op_sym:tt, $op_trait:ident, $op_fn:ident) => (
        impl<T: Copy + Debug> $op_trait<Coord<T>> for Coord<T>
        where
            T: $op_trait<Output = T>,
        {
            type Output = Coord<T>;
            fn $op_fn(self, rhs: Coord<T>) -> Self::Output {
                Coord {
                    x: self.x $op_sym rhs.x,
                    y: self.y $op_sym rhs.y,
                }
            }
        }
    )
}

coord_coord_op!(+, Add, add);
coord_coord_op!(-, Sub, sub);

impl<T: Copy + Debug> Mul<T> for Coord<T>
where
    T: Mul<T, Output = T>,
{
    type Output = Coord<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Coord {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let a = Coord { x: 1, y: 2 };
        let b = Coord { x: 3, y: 5 };
        assert_eq!(Coord::new(4, 7), a + b);
    }

    #[test]
    fn sub() {
        let a = Coord { x: 1, y: 2 };
        let b = Coord { x: 3, y: 5 };
        assert_eq!(Coord::new(-2, -3), a - b);
    }

    #[test]
    fn mul() {
        assert_eq!(Coord::new(3, -9), Coord::new(1, -3) * 3);
    }
}
