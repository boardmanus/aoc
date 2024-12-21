use std::ops::{Add, Sub};

use approx::AbsDiffEq;
use num_traits::{Float, FromPrimitive, Num};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Vec2d<Scalar: Num> {
    pub x: Scalar,
    pub y: Scalar,
}

impl<Scalar: Num> Vec2d<Scalar> {
    pub fn new(x: Scalar, y: Scalar) -> Vec2d<Scalar> {
        Vec2d { x, y }
    }
}

impl<Scalar: Num> Add for Vec2d<Scalar> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2d::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<Scalar: Num> Sub for Vec2d<Scalar> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2d::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<Scalar: Float + FromPrimitive> AbsDiffEq for Vec2d<Scalar> {
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
            Vec2d::<usize>::new(1, 2) + Vec2d::<usize>::new(4, 5),
            Vec2d::<usize>::new(5, 7)
        );
        assert_abs_diff_eq!(
            Vec2d::<f64>::new(1.0, 2.1) + Vec2d::<f64>::new(-4.1, 5.2),
            Vec2d::<f64>::new(-3.1, 7.3)
        );
    }
}
