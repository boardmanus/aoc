use std::{
    cmp::Ordering,
    fmt::Display,
    ops::{Add, Sub},
};

use approx::AbsDiffEq;
use num_traits::{Float, FromPrimitive, Num, Signed};

use crate::vec3d::Vec3d;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Pos3d<Scalar: Num> {
    pub x: Scalar,
    pub y: Scalar,
    pub z: Scalar,
}

impl<Scalar: Num + Display> Display for Pos3d<Scalar> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl<Scalar: Num + PartialOrd> PartialOrd for Pos3d<Scalar> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.z.partial_cmp(&other.z) {
            Some(Ordering::Equal) => match self.y.partial_cmp(&other.y) {
                Some(Ordering::Equal) => self.x.partial_cmp(&other.x),
                ord => ord,
            },
            ord => return ord,
        }
    }
}

impl<Scalar: Num + Ord + Eq> Ord for Pos3d<Scalar> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.z.cmp(&other.y) {
            Ordering::Equal => match self.y.cmp(&other.y) {
                Ordering::Equal => self.x.cmp(&other.x),
                ord => ord,
            },
            ord => ord,
        }
    }
}

impl<Scalar: Num> Pos3d<Scalar> {
    pub fn new(x: Scalar, y: Scalar, z: Scalar) -> Pos3d<Scalar> {
        Pos3d { x, y, z }
    }
}

impl<Scalar: Num + Signed + Copy> Pos3d<Scalar> {
    pub fn taxi_distance(&self, other: &Self) -> Scalar {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

impl<Scalar: Num> Add<Vec3d<Scalar>> for Pos3d<Scalar> {
    type Output = Self;

    fn add(self, rhs: Vec3d<Scalar>) -> Self::Output {
        Pos3d::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl<Scalar: Num> Sub<Vec3d<Scalar>> for Pos3d<Scalar> {
    type Output = Self;

    fn sub(self, rhs: Vec3d<Scalar>) -> Self::Output {
        Pos3d::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<Scalar: Num + Copy + From<i64>> Sub<Pos3d<Scalar>> for Pos3d<Scalar> {
    type Output = Vec3d<Scalar>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3d::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<Scalar: Float + FromPrimitive> AbsDiffEq for Pos3d<Scalar> {
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
            Pos3d::<usize>::new(1, 2, 3) + Vec3d::<usize>::new(4, 5, 6),
            Pos3d::<usize>::new(5, 7, 9)
        );
        assert_abs_diff_eq!(
            Pos3d::<f64>::new(1.0, 2.1, 3.2) + Vec3d::<f64>::new(-4.1, 5.2, -6.3),
            Pos3d::<f64>::new(-3.1, 7.3, -3.1)
        );
    }

    #[test]
    fn test_cmp() {
        assert!(Pos3d::<usize>::new(1, 2, 3) > Pos3d::<usize>::new(3, 2, 1));
        assert!(Pos3d::<i64>::new(1, 2, 3) > Pos3d::<i64>::new(2, 1, 3));
        assert!(Pos3d::<i64>::new(1, 2, 3) > Pos3d::<i64>::new(0, 2, 3));
        assert!(Pos3d::<usize>::new(1, 2, 3) < Pos3d::<usize>::new(1, 2, 4));
        assert!(Pos3d::<usize>::new(1, 2, 3) < Pos3d::<usize>::new(1, 3, 3));
        assert!(Pos3d::<usize>::new(1, 2, 3) < Pos3d::<usize>::new(2, 2, 3));
        assert!(Pos3d::<i64>::new(1, 2, 3) == Pos3d::<i64>::new(1, 2, 3));
        assert!(Pos3d::<i64>::new(1, 2, 3) >= Pos3d::<i64>::new(1, 2, 3));
        assert!(Pos3d::<i64>::new(1, 2, 3) <= Pos3d::<i64>::new(1, 2, 3));
        assert!(Pos3d::<i64>::new(1, 2, 3) != Pos3d::<i64>::new(-1, 2, 3));
        assert!(Pos3d::<i64>::new(1, 2, 3) > Pos3d::<i64>::new(-1, 2, 3));
        assert!(Pos3d::<i64>::new(1, 2, 3) > Pos3d::<i64>::new(1, -2, 3));
        assert!(Pos3d::<i64>::new(1, 2, 3) > Pos3d::<i64>::new(1, 2, -3));
    }
}
