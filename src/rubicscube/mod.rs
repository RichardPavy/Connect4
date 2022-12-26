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

#[derive(Clone, Default)]
struct Coord3 {
    array: [i8; 3],
}

impl Coord3 {
    fn x(&self) -> i8 {
        self.array[0]
    }
    fn y(&self) -> i8 {
        self.array[1]
    }
    fn z(&self) -> i8 {
        self.array[2]
    }

    fn x_mut(&mut self) -> &mut i8 {
        &mut self.array[0]
    }
    fn y_mut(&mut self) -> &mut i8 {
        &mut self.array[1]
    }
    fn z_mut(&mut self) -> &mut i8 {
        &mut self.array[2]
    }

    const fn new(x: i8, y: i8, z: i8) -> Coord3 {
        Coord3 { array: [x, y, z] }
    }
}

static UP: Coord3 = Coord3::new(1, 0, 0);
static DOWN: Coord3 = Coord3::new(-1, 0, 0);

static LEFT: Coord3 = Coord3::new(0, 1, 0);
static RIGHT: Coord3 = Coord3::new(0, -1, 0);

static FRONT: Coord3 = Coord3::new(0, 0, 1);
static BACK: Coord3 = Coord3::new(0, 0, -1);

struct Facet {
    board: ArrayBoard<3, 3, Color>,
}

impl Facet {
    pub fn new() -> Self {
        Self {
            board: ArrayBoard::generate(|point| ' '),
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
            facets: std::array::from_fn(|i| Facet::new()),
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
    use crate::shared::board::board_get_set::BoardGet;
    use crate::shared::coord::point::Point;

    use super::Cube;
    use super::UP;

    #[test]
    fn get_facet() {
        let cube = Cube::new();
        assert_eq!(*cube.get(&UP).board.get(&Point::new(1, 1)), ' ');
    }
}
