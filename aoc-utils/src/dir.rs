use enum_iterator::{all, next_cycle, previous_cycle, reverse_all, All, ReverseAll, Sequence};

use crate::vec2d::Vec2d;

pub type DirVec = Vec2d<i64>;

pub trait Dir<T> {
    fn cw() -> All<T>;
    fn ccw() -> ReverseAll<T>;
    fn rotate_cw(&self) -> T;
    fn rotate_ccw(&self) -> T;
    fn from_i(i: usize) -> T;
    fn to_i(&self) -> usize;
    fn to_vec2d(&self) -> DirVec;
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

    fn from_i(i: usize) -> Dir8 {
        match i % 8 {
            0 => Dir8::N,
            1 => Dir8::NE,
            2 => Dir8::E,
            3 => Dir8::SE,
            4 => Dir8::S,
            5 => Dir8::SW,
            6 => Dir8::W,
            7 => Dir8::NW,
            _ => panic!(),
        }


    }
    fn to_i(&self) -> usize {
        match self {
            Dir8::N => 0,
            Dir8::NE => 1,
            Dir8::E => 2,
            Dir8::SE => 3,
            Dir8::S => 4,
            Dir8::SW => 5,
            Dir8::W => 6,
            Dir8::NW => 7,
        }
    }

    fn to_vec2d(&self) -> DirVec {
        match self {
            Dir8::N => Vec2d { x: 0, y: -1 },
            Dir8::NE => Vec2d { x: 1, y: -1 },
            Dir8::E => Vec2d { x: 1, y: 0 },
            Dir8::SE => Vec2d { x: 1, y: 1 },
            Dir8::S => Vec2d { x: 0, y: 1 },
            Dir8::SW => Vec2d { x: -1, y: 1 },
            Dir8::W => Vec2d { x: -1, y: 0 },
            Dir8::NW => Vec2d { x: -1, y: -1 },
        }
    }
}

impl Into<DirVec> for Dir8 {
    fn into(self) -> DirVec {
        self.to_vec2d()
    }
}

impl From<usize> for Dir8 {
    fn from(i: usize) -> Dir8 {
        Dir8::from_i(i)
    }
}

impl Into<usize> for Dir8 {
    fn into(self) -> usize {
        self.to_i()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Sequence)]
pub enum Dir4 {
    N,
    E,
    S,
    W,
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
    
    fn from_i(i: usize) -> Dir4 {
        
        match i % 4 {
            0 => Dir4::N,
            1 => Dir4::E,
            2 => Dir4::S,
            3 => Dir4::W,
            _ => panic!(),
        }
    }
    
    fn to_i(&self) -> usize {
        match self {
            Dir4::N => 0,
            Dir4::E => 1,
            Dir4::S => 2,
            Dir4::W => 3,
        }
    }

    fn to_vec2d(&self) -> DirVec {
        match self {
            Dir4::N => Vec2d { x: 0, y: -1 },
            Dir4::E => Vec2d { x: 1, y: 0 },
            Dir4::S => Vec2d { x: 0, y: 1 },
            Dir4::W => Vec2d { x: -1, y: 0 },
        }
    }
}

impl Into<DirVec> for Dir4 {
    fn into(self) -> DirVec {
        self.to_vec2d()
    }
}

impl From<usize> for Dir4 {
    fn from(i: usize) -> Dir4 {
        Dir4::from_i(i)
    }
}

impl Into<usize> for Dir4 {
    fn into(self) -> usize {
        self.to_i()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_vec2d() {
            assert_eq!(Dir4::N.to_vec2d(), Vec2d { x: 0, y: -1 });
        assert_eq!(Dir4::E.to_vec2d(), Vec2d { x: 1, y: 0 });
        assert_eq!(Dir4::S.to_vec2d(), Vec2d { x: 0, y: 1 });
        assert_eq!(Dir4::W.to_vec2d(), Vec2d { x: -1, y: 0 });
    }
}