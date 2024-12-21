use num_traits::Num;

pub struct Pos2d<Scalar: Num> {
    x: Scalar,
    y: Scalar,
}

impl<Scalar: Num> Pos2d<Scalar> {
    pub fn new(x: Scalar, y: Scalar) -> Pos2d<Scalar> {
        Pos2d { x, y }
    }
}

