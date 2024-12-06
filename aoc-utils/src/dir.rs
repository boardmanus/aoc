use crate::grid::Index;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Dir {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Dir {
    pub fn grid_dir(&self) -> Index {
        match self {
            Dir::N => Index(0, -1),
            Dir::NE => Index(1, -1),
            Dir::E => Index(1, 0),
            Dir::SE => Index(1, 1),
            Dir::S => Index(0, 1),
            Dir::SW => Index(-1, 1),
            Dir::W => Index(-1, 0),
            Dir::NW => Index(-1, -1),
        }
    }

    pub fn rotate_cw(&self) -> Dir {
        match self {
            Dir::N => Dir::E,
            Dir::NE => Dir::SE,
            Dir::E => Dir::S,
            Dir::SE => Dir::SW,
            Dir::S => Dir::W,
            Dir::SW => Dir::NW,
            Dir::W => Dir::N,
            Dir::NW => Dir::NE,
        }
    }
}
