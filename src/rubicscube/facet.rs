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

pub const ALL_FACETS: &[Facet] = &[
    Facet::North,
    Facet::South,
    Facet::East,
    Facet::West,
    Facet::Top,
    Facet::Bottom,
];

impl Facet {
    pub const fn opposite(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
            Self::Top => Self::Bottom,
            Self::Bottom => Self::Top,
        }
    }

    pub fn neighbors(self) -> impl Iterator<Item = Self> {
        let opposite = self.opposite();
        ALL_FACETS
            .iter()
            .map(|f| *f)
            .filter(move |f| *f != opposite)
    }

    pub fn pitch(self) -> Self {
        match self {
            Self::North => Self::Bottom,
            Self::South => Self::Top,
            Self::East => Self::East,
            Self::West => Self::West,
            Self::Top => Self::North,
            Self::Bottom => Self::South,
        }
    }

    pub fn yaw(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::South => Self::West,
            Self::East => Self::South,
            Self::West => Self::North,
            Self::Top => Self::Top,
            Self::Bottom => Self::Bottom,
        }
    }

    pub fn roll(self) -> Self {
        match self {
            Self::North => Self::North,
            Self::South => Self::South,
            Self::East => Self::Bottom,
            Self::West => Self::Top,
            Self::Top => Self::East,
            Self::Bottom => Self::West,
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

    #[test]
    fn neighbors() {
        for facet in ALL_FACETS {
            assert!(facet
                .neighbors()
                .collect::<Vec<Facet>>()
                .contains(&facet.pitch()));
            assert!(facet
                .neighbors()
                .collect::<Vec<Facet>>()
                .contains(&facet.yaw()));
            assert!(facet
                .neighbors()
                .collect::<Vec<Facet>>()
                .contains(&facet.roll()));
        }
    }

    #[test]
    fn circular() {
        for facet in ALL_FACETS {
            assert_eq!(*facet, facet.pitch().pitch().pitch().pitch());
            assert_eq!(*facet, facet.yaw().yaw().yaw().yaw());
            assert_eq!(*facet, facet.roll().roll().roll().roll());
        }
    }
}
