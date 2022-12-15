use crate::shared::board::board_to_string::AsChar;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Symbol {
    Empty,
    Red,
    Yellow,
}

impl Symbol {
    pub fn other(self) -> Symbol {
        match self {
            Symbol::Empty => Symbol::Empty,
            Symbol::Yellow => Symbol::Red,
            Symbol::Red => Symbol::Yellow,
        }
    }
}

impl AsChar for Symbol {
    fn as_char(&self) -> char {
        match self {
            Symbol::Empty => ' ',
            Symbol::Yellow => 'Y',
            Symbol::Red => 'R',
        }
    }
}

impl Default for Symbol {
    fn default() -> Self {
        Symbol::Empty
    }
}
