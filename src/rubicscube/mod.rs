/*
A----B----C
  \    \    \
   D----E----F
    \    \    \
      G----H----I

J----K----L
  \    \    \
   M----N----O
    \    \    \
      P----Q----R

S----T----U
  \    \    \
   V----W----X
    \    \    \
      Y----Z----1


                +---+---+---+
                |   |   |   |
                +---+---+---+
                |   |   |   |
                +---+---+---+
                |   |   |   |
                +---+---+---+

+---+---+---+   +---+---+---+   +---+---+---+   +---+---+---+
|   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |
+---+---+---+   +---+---+---+   +---+---+---+   +---+---+---+
|   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |
+---+---+---+   +---+---+---+   +---+---+---+   +---+---+---+
|   |   |   |   |   |   |   |   |   |   |   |   |   |   |   |
+---+---+---+   +---+---+---+   +---+---+---+   +---+---+---+

                +---+---+---+
                |   |   |   |
                +---+---+---+
                |   |   |   |
                +---+---+---+
                |   |   |   |
                +---+---+---+
*/

use crate::shared::board::array_board::ArrayBoard;

use self::coord3::Coord3;

mod coord3;

struct Facet {
    board: ArrayBoard<3, 3, Color>,
}

impl Facet {
    pub fn new() -> Self {
        Self {
            board: ArrayBoard::generate(|_point| ' '),
        }
    }
}

type Color = char;

struct Cube {
    facets: [Facet; 6],
}

impl Cube {
    pub fn new() -> Self {
        Self {
            facets: std::array::from_fn(|_i| Facet::new()),
        }
    }

    fn get(&self, coord: &Coord3) -> &Facet {
        let idx = match 3 + coord.x() + 2 * coord.y() + 3 * coord.z() {
            idx if idx > 3 => idx - 3,
            idx => idx,
        };
        &self.facets[idx as usize]
    }
}

enum Xyz {
    A,
    B,
    C,
}

#[cfg(test)]
mod tests {
    use crate::rubicscube::coord3;
    use crate::shared::board::board_get_set::BoardGet;
    use crate::shared::coord::point::Point;

    use super::Cube;

    #[test]
    fn get_facet() {
        let cube = Cube::new();
        assert_eq!(*cube.get(&coord3::UP).board.get(&Point::new(1, 1)), ' ');
    }
}
