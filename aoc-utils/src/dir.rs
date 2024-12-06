use enum_iterator::Sequence;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Sequence)]
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
