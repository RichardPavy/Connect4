use crate::shared::board::board_to_string::AsChar;

use super::symbol::Symbol;

#[derive(Default, Debug, PartialEq, Eq)]
pub struct Connect4Cell {
    pub symbol: Symbol,
}

impl AsChar for Connect4Cell {
    fn as_char(&self) -> char {
        self.symbol.as_char()
    }
}
