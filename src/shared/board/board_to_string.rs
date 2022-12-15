use crate::shared::coord::point::Point;

use super::board_get_set::BoardGet;
use super::board_size::BoardSize;
use super::board_view::BoardView;

impl<'b, TBoard, CellView, ViewFn> std::fmt::Display for BoardView<'b, TBoard, ViewFn>
where
    TBoard: BoardSize + BoardGet,
    ViewFn: Fn(&<TBoard as BoardGet>::Value) -> &CellView,
    CellView: AsChar,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();

        let separator_line: &str = &{
            let mut s = String::new();
            s.push_str("----");
            (0..self.width()).for_each(|_| {
                s.push_str(&format!("+---"));
            });
            s.push('\n');
            s
        };

        {
            // Headers
            s.push_str("    ");
            (1..=self.width()).for_each(|i| {
                s.push_str(&format!("| {i} "));
            });
            s.push('\n');
        }

        (0..self.height()).for_each(|j| {
            s.push_str(separator_line);
            s.push_str(&format!(
                " {}  ",
                char::from_u32(j as u32 + 'A' as u32).unwrap()
            ));
            (0..self.width()).for_each(|i| {
                s.push_str(&format!("| {} ", self.get(&Point::new(i, j)).as_char()));
            });
            s.push('\n');
        });

        f.write_str(&s)
    }
}

pub trait AsChar {
    fn as_char(&self) -> char;
}

impl AsChar for char {
    fn as_char(&self) -> char {
        *self
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
