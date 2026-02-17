#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl Direction {
    pub const NORTH: Self = Self::North;
    pub const SOUTH: Self = Self::South;
    pub const EAST: Self = Self::East;
    pub const WEST: Self = Self::West;
    pub const NORTHEAST: Self = Self::NorthEast;
    pub const NORTHWEST: Self = Self::NorthWest;
    pub const SOUTHEAST: Self = Self::SouthEast;
    pub const SOUTHWEST: Self = Self::SouthWest;

    pub const fn row_delta(self) -> i32 {
        self.delta().0
    }

    pub const fn col_delta(self) -> i32 {
        self.delta().1
    }

    pub const fn delta(self) -> (i32, i32) {
        match self {
            Self::North => (-1, 0),
            Self::South => (1, 0),
            Self::East => (0, 1),
            Self::West => (0, -1),
            Self::NorthEast => (-1, 1),
            Self::NorthWest => (-1, -1),
            Self::SouthEast => (1, 1),
            Self::SouthWest => (1, -1),
        }
    }
}
