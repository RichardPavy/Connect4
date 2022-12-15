use crate::shared::coord::point::Point;

use super::board_get_set::BoardGet;
use super::board_size::BoardSize;
use super::board_to_string::AsChar;

pub struct BoardView<'b, TBoard: ?Sized, ViewFn> {
    board: &'b TBoard,
    view_fn: ViewFn,
}

pub trait AsBoardView<CellView, ViewFn>
where
    Self: BoardSize + BoardGet,
    ViewFn: Fn(&<Self as BoardGet>::Value) -> &CellView,
    CellView: AsChar,
{
    fn as_view<'a>(&'a self, view_fn: ViewFn) -> BoardView<'a, Self, ViewFn> {
        BoardView {
            board: self,
            view_fn,
        }
    }
}

impl<'b, TBoard: BoardGet + BoardSize, ViewFn> BoardSize for BoardView<'b, TBoard, ViewFn> {
    fn width(&self) -> i32 {
        self.board.width()
    }

    fn height(&self) -> i32 {
        self.board.height()
    }
}

impl<'b, TBoard, CellView, ViewFn> BoardGet for BoardView<'b, TBoard, ViewFn>
where
    TBoard: BoardGet,
    ViewFn: Fn(&<TBoard as BoardGet>::Value) -> &CellView,
{
    type Value = CellView;
    fn get(&self, point: &Point) -> &CellView {
        (self.view_fn)(self.board.get(point))
    }
}

#[cfg(test)]
mod tests {
    use crate::shared::board::array_board::ArrayBoard;

    #[test]
    fn to_string() {
        type TestBoard = ArrayBoard<3, 5, char>;
        let board = TestBoard::generate(|point| {
            char::from_u32('a' as u32 + (point.x - point.y).abs() as u32).unwrap()
        });
        println!("{}", board);
        assert_eq!(
            r#"
    | 1 | 2 | 3 
----+---+---+---
 A  | a | b | c 
----+---+---+---
 B  | b | a | b 
----+---+---+---
 C  | c | b | a 
----+---+---+---
 D  | d | c | b 
----+---+---+---
 E  | e | d | c 
"#,
            format!("\n{board}")
        );
    }
}
