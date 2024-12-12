use enum_iterator::{all, next_cycle, previous_cycle, reverse_all, All, ReverseAll, Sequence};

pub trait Dir<T> {
    fn cw() -> All<T>;
    fn ccw() -> ReverseAll<T>;
    fn rotate_cw(&self) -> T;
    fn rotate_ccw(&self) -> T;
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Sequence)]
pub enum Dir8 {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Dir<Dir8> for Dir8 {
    fn cw() -> All<Dir8> {
        all::<Dir8>()
    }

    fn ccw() -> ReverseAll<Dir8> {
        reverse_all::<Dir8>()
    }

    fn rotate_cw(&self) -> Dir8 {
        next_cycle::<Dir8>(self)
    }

    fn rotate_ccw(&self) -> Dir8 {
        previous_cycle::<Dir8>(self)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Sequence)]
pub enum Dir4 {
    N,
    E,
    S,
    W,
}

impl Dir4 {
    pub fn index(&self) -> usize {
        match self {
            Dir4::N => 0,
            Dir4::E => 1,
            Dir4::S => 2,
            Dir4::W => 3,
        }
    }
}

impl Dir<Dir4> for Dir4 {
    fn cw() -> All<Dir4> {
        all::<Dir4>()
    }

    fn ccw() -> ReverseAll<Dir4> {
        reverse_all::<Dir4>()
    }

    fn rotate_cw(&self) -> Dir4 {
        next_cycle::<Dir4>(self)
    }

    fn rotate_ccw(&self) -> Dir4 {
        previous_cycle::<Dir4>(self)
    }
}
