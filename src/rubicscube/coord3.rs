#[derive(Clone, Default)]
pub struct Coord3 {
    array: [i8; 3],
}

impl Coord3 {
    pub fn x(&self) -> i8 {
        self.array[0]
    }
    pub fn y(&self) -> i8 {
        self.array[1]
    }
    pub fn z(&self) -> i8 {
        self.array[2]
    }

    pub fn x_mut(&mut self) -> &mut i8 {
        &mut self.array[0]
    }
    pub fn y_mut(&mut self) -> &mut i8 {
        &mut self.array[1]
    }
    pub fn z_mut(&mut self) -> &mut i8 {
        &mut self.array[2]
    }

    pub const fn new(x: i8, y: i8, z: i8) -> Coord3 {
        Coord3 { array: [x, y, z] }
    }
}

pub static UP: Coord3 = Coord3::new(1, 0, 0);
pub static DOWN: Coord3 = Coord3::new(-1, 0, 0);

pub static LEFT: Coord3 = Coord3::new(0, 1, 0);
pub static RIGHT: Coord3 = Coord3::new(0, -1, 0);

pub static FRONT: Coord3 = Coord3::new(0, 0, 1);
pub static BACK: Coord3 = Coord3::new(0, 0, -1);
