use std::{
    cmp::Ordering,
    ops::{Add, Sub},
};

use approx::AbsDiffEq;
use num_traits::{Float, FromPrimitive, Num, PrimInt, Signed};

use crate::vec2d::Vec2d;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Pos2d<Scalar: Num> {
    pub x: Scalar,
    pub y: Scalar,
}

impl<Scalar: Num> Pos2d<Scalar> {
    pub fn new(x: Scalar, y: Scalar) -> Pos2d<Scalar> {
        Pos2d { x, y }
    }
}

impl<Scalar: Num + Signed + Copy> Pos2d<Scalar> {
    pub fn taxi_distance(&self, other: &Self) -> Scalar {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl<Scalar: Num> Add<Vec2d<Scalar>> for Pos2d<Scalar> {
    type Output = Self;

    fn add(self, rhs: Vec2d<Scalar>) -> Self::Output {
        Pos2d::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<Scalar: Num> Sub<Vec2d<Scalar>> for Pos2d<Scalar> {
    type Output = Self;

    fn sub(self, rhs: Vec2d<Scalar>) -> Self::Output {
        Pos2d::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<Scalar: PrimInt> PartialOrd for Pos2d<Scalar> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.x == other.x {
            Some(self.y.cmp(&other.y))
        } else {
            Some(self.x.cmp(&other.x))
        }
    }
}

impl<Scalar: Float + FromPrimitive> AbsDiffEq for Pos2d<Scalar> {
    type Epsilon = Scalar;

    fn default_epsilon() -> Self::Epsilon {
        Scalar::epsilon() * Scalar::from_f64(16.0).unwrap()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        let dx = (self.x - other.x).abs();
        let dy = (self.y - other.y).abs();
        dx + dy <= epsilon
    }
}

#[cfg(test)]
mod tests {

    use approx::assert_abs_diff_eq;

    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(
            Pos2d::<usize>::new(1, 2) + Vec2d::<usize>::new(4, 5),
            Pos2d::<usize>::new(5, 7)
        );
        assert_abs_diff_eq!(
            Pos2d::<f64>::new(1.0, 2.1) + Vec2d::<f64>::new(-4.1, 5.2),
            Pos2d::<f64>::new(-3.1, 7.3)
        );
    }

    #[test]
    fn test_cmp() {
        assert!(Pos2d::<usize>::new(1, 2) < Pos2d::<usize>::new(2, 1));
        assert!(Pos2d::<usize>::new(1, 2) < Pos2d::<usize>::new(1, 3));
        assert!(Pos2d::<usize>::new(1, 2) < Pos2d::<usize>::new(2, 2));
        assert!(Pos2d::<i64>::new(1, 2) < Pos2d::<i64>::new(2, 1));
        assert!(Pos2d::<i64>::new(1, 2) < Pos2d::<i64>::new(1, 3));
        assert!(Pos2d::<i64>::new(1, 2) < Pos2d::<i64>::new(2, 2));
        assert!(Pos2d::<i64>::new(1, 2) == Pos2d::<i64>::new(1, 2));
        assert!(Pos2d::<i64>::new(1, 2) != Pos2d::<i64>::new(-1, 2));
        assert!(Pos2d::<i64>::new(1, 2) > Pos2d::<i64>::new(-1, 2));
    }
}
