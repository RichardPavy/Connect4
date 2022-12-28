struct RubicksPoint {}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Facet {
    North,
    South,
    East,
    West,
    Top,
    Bottom,
}

const ALL_FACETS: &[Facet] = &[
    Facet::North,
    Facet::South,
    Facet::East,
    Facet::West,
    Facet::Top,
    Facet::Bottom,
];

impl Facet {
    pub const fn opposite(self) -> Facet {
        match self {
            Facet::North => Facet::South,
            Facet::South => Facet::North,
            Facet::East => Facet::West,
            Facet::West => Facet::East,
            Facet::Top => Facet::Bottom,
            Facet::Bottom => Facet::Top,
        }
    }

    pub fn neighbors(self) -> impl Iterator<Item = Facet> {
        let opposite = self.opposite();
        ALL_FACETS
            .iter()
            .map(|f| *f)
            .filter(move |f| *f != opposite)
    }

    pub fn pitch(self) -> Facet {
        match self {
            Facet::North => Facet::Bottom,
            Facet::South => Facet::Top,
            Facet::East => Facet::East,
            Facet::West => Facet::West,
            Facet::Top => Facet::North,
            Facet::Bottom => Facet::South,
        }
    }

    pub fn yaw(self) -> Facet {
        match self {
            Facet::North => Facet::East,
            Facet::South => Facet::West,
            Facet::East => Facet::South,
            Facet::West => Facet::North,
            Facet::Top => Facet::Top,
            Facet::Bottom => Facet::Bottom,
        }
    }

    pub fn roll(self) -> Facet {
        match self {
            Facet::North => Facet::North,
            Facet::South => Facet::South,
            Facet::East => Facet::Bottom,
            Facet::West => Facet::Top,
            Facet::Top => Facet::East,
            Facet::Bottom => Facet::West,
        }
    }
}

#[cfg(test)]
#[allow(non_upper_case_globals)]
mod tests {
    use super::Facet;
    use super::ALL_FACETS;

    const North: Facet = Facet::North;
    const South: Facet = Facet::South;
    const East: Facet = Facet::East;
    const West: Facet = Facet::West;
    const Top: Facet = Facet::Top;
    const Bottom: Facet = Facet::Bottom;

    #[test]
    fn pitch() {
        let actual: Vec<(Facet, Facet)> = ALL_FACETS.iter().map(|f| (*f, f.pitch())).collect();
        let expected = vec![
            (North, Bottom),
            (South, Top),
            (East, East),
            (West, West),
            (Top, North),
            (Bottom, South),
        ];
        assert_eq!(expected, actual);
    }

    #[test]
    fn yaw() {
        let actual: Vec<(Facet, Facet)> = ALL_FACETS.iter().map(|f| (*f, f.yaw())).collect();
        let expected = vec![
            (North, East),
            (South, West),
            (East, South),
            (West, North),
            (Top, Top),
            (Bottom, Bottom),
        ];
        assert_eq!(expected, actual);
    }

    #[test]
    fn roll() {
        let actual: Vec<(Facet, Facet)> = ALL_FACETS.iter().map(|f| (*f, f.roll())).collect();
        let expected = vec![
            (North, North),
            (South, South),
            (East, Bottom),
            (West, Top),
            (Top, East),
            (Bottom, West),
        ];
        assert_eq!(expected, actual);
    }
}
