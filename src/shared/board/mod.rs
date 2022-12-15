use std::marker::PhantomData;

use self::board_get_set::BoardGet;
use self::board_size::BoardSize;
use self::board_to_string::AsChar;
use self::board_view::AsBoardView;

pub mod array_board;
pub mod board_all_points;
pub mod board_corners;
pub mod board_for_all_points;
pub mod board_get_set;
pub mod board_iterate;
pub mod board_lines;
pub mod board_size;
pub mod board_to_string;
pub mod board_view;

pub struct Board<Array, Value> {
    board: Array,
    _phantom: PhantomData<Value>,
}

impl<Array, Value> Default for Board<Array, Value>
where
    Array: Default,
{
    fn default() -> Self {
        Self {
            board: Default::default(),
            _phantom: PhantomData,
        }
    }
}

impl<Array, Value> Board<Array, Value> {
    fn new(array: Array) -> Self {
        Self {
            board: array,
            _phantom: PhantomData,
        }
    }
}

impl<'b, Array, Value, CellView, ViewFn> AsBoardView<CellView, ViewFn> for Board<Array, Value>
where
    Self: BoardSize + BoardGet,
    ViewFn: Fn(&<Self as BoardGet>::Value) -> &CellView,
    CellView: AsChar,
{
}

impl<Array, Value> std::fmt::Display for Board<Array, Value>
where
    Self: BoardGet<Value = Value> + BoardSize,
    Value: AsChar,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_view(|c| c).fmt(f)
    }
}
