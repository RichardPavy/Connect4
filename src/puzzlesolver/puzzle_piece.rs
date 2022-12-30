use crate::shared::board::board_to_string::AsChar;
use crate::shared::coord::point::Point;

#[derive(Debug, PartialEq, Eq)]
pub(super) enum PuzzlePiece {
    Blank { char: char },
    Shape { shape_idx: ShapeIdx },
}

pub(super) type ShapeIdx = u32;

impl PuzzlePiece {
    pub fn is_blank(&self) -> bool {
        match self {
            PuzzlePiece::Blank { .. } => true,
            PuzzlePiece::Shape { .. } => false,
        }
    }

    pub fn shape(shape_idx: ShapeIdx) -> Self {
        Self::Shape { shape_idx }
    }

    pub fn blank(point: &Point) -> Self {
        if (point.x + point.y) % 2 == 0 {
            Self::Blank { char: 'X' }
        } else {
            Self::Blank { char: 'O' }
        }
    }

    pub fn blank_char(char: char) -> Self {
        Self::Blank { char }
    }
}

impl AsChar for PuzzlePiece {
    fn as_char(&self) -> char {
        match self {
            PuzzlePiece::Blank { char } => *char,
            PuzzlePiece::Shape { shape_idx } => ('a' as u8 + *shape_idx as u8) as char,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::board::board_to_string::AsChar;
    use crate::shared::coord::point::Point;

    use super::PuzzlePiece;

    #[test]
    fn puzzle_piece() {
        assert_eq!('a', PuzzlePiece::shape(0).as_char());
        assert_eq!('b', PuzzlePiece::shape(1).as_char());
        assert_eq!('z', PuzzlePiece::shape(25).as_char());
        assert_eq!('X', PuzzlePiece::blank(&Point::new(1, 1)).as_char());
        assert_eq!('O', PuzzlePiece::blank(&Point::new(1, 2)).as_char());
    }
}
