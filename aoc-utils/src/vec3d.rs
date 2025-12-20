use std::{
    fmt::Display,
    ops::{Add, Mul, Neg, Sub},
};

use crate::vecnd::VecSize;
use approx::AbsDiffEq;
use num_traits::{Float, FromPrimitive, Num, Signed};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash, Copy, Clone)]
pub struct Vec3d<Scalar: Num> {
    pub x: Scalar,
    pub y: Scalar,
    pub z: Scalar,
}

impl<Scalar: Num> Vec3d<Scalar> {
    pub const fn new(x: Scalar, y: Scalar, z: Scalar) -> Vec3d<Scalar> {
        Vec3d { x, y, z }
    }
}

impl<Scalar: Num + Display> Display for Vec3d<Scalar> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{},{}]", self.x, self.y, self.z)
    }
}

impl<Scalar: Signed + Copy> VecSize<Scalar> for Vec3d<Scalar> {
    fn mag_sqr(&self) -> Scalar {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn manhattan(&self) -> Scalar {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl<Scalar: Num> Add for Vec3d<Scalar> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3d::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<Scalar: Num> Sub for Vec3d<Scalar> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3d::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<Scalar: Num + Copy> Mul<Scalar> for Vec3d<Scalar> {
    type Output = Self;

    fn mul(self, rhs: Scalar) -> Self::Output {
        Vec3d::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl<Scalar: Num + Neg<Output = Scalar> + Copy> Neg for Vec3d<Scalar> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3d::new(-self.x, -self.y, -self.z)
    }
}

impl<Scalar: Float + FromPrimitive> AbsDiffEq for Vec3d<Scalar> {
    type Epsilon = Scalar;

    fn default_epsilon() -> Self::Epsilon {
        Scalar::epsilon() * Scalar::from_f64(16.0).unwrap()
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        let dx = (self.x - other.x).abs();
        let dy = (self.y - other.y).abs();
        let dz = (self.z - other.z).abs();
        dx + dy + dz <= epsilon
    }
}

#[cfg(test)]
mod tests {

    use approx::assert_abs_diff_eq;

    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(
            Vec3d::<usize>::new(1, 2, 3) + Vec3d::<usize>::new(4, 5, 6),
            Vec3d::<usize>::new(5, 7, 9)
        );
        assert_abs_diff_eq!(
            Vec3d::<f64>::new(1.0, 2.1, 3.2) + Vec3d::<f64>::new(-4.1, 5.2, 6.3),
            Vec3d::<f64>::new(-3.1, 7.3, 9.5)
        );
    }
}
